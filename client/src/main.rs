extern crate api;
#[macro_use]
extern crate clap;
extern crate lettre;
#[macro_use]
extern crate log;
extern crate protobuf;
extern crate simplelog;
extern crate zmq;

use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result};
use std::path::Path;

use api::service::*;
use clap::{Arg, ArgMatches, App};
use lettre::email::{Email, EmailBuilder};
use lettre::transport::EmailTransport;
use lettre::transport::smtp::SmtpTransportBuilder;
use protobuf::parse_from_bytes;
use simplelog::{Config, SimpleLogger};
use zmq::Socket;
use zmq::Result as ZmqResult;

fn send_email(smtp_host: &str, email: Email) -> Result<()> {
    let mut transport = SmtpTransportBuilder::new(smtp_host)
        .map_err(|e| Error::new(ErrorKind::Other, e))?
        .build();
    transport.get_ehlo().map_err(
        |e| Error::new(ErrorKind::Other, e),
    )?;
    transport.send(email).map_err(
        |e| Error::new(ErrorKind::Other, e),
    )?;
    Ok(())
}

fn build_email(to: &Vec<&str>, from: &str, usage_info: &str) -> Result<Email> {
    let mut builder = EmailBuilder::new();
    for t in to {
        builder.add_to((t.as_ref(), ""));
    }
    builder.add_from(from);
    builder.set_subject("Ceph usage information");
    let email = builder.text(usage_info).build().map_err(|e| {
        Error::new(ErrorKind::Other, e)
    })?;
    Ok(email)
}

/*
CLI client to call functions over RPC
*/

fn connect(host: &str, port: &str) -> ZmqResult<Socket> {
    debug!("Starting zmq sender with version({:?})", zmq::version());
    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ)?;
    debug!("Connecting to: {}:{}", host, port);
    assert!(
        requester
            .connect(&format!("tcp://{}:{}", host, port))
            .is_ok()
    );
    debug!("Connected");

    Ok(requester)
}

fn transform_csv(cluster_info: &ClusterUsage, region: &str) -> String {
    let mut buff = String::new();
    buff.push_str(
        "host,total_kb,avail_kb,used_kb,block_kb,obj_kb,glance_kb,block_size,obj_size,glance_size",
    );
    buff.push_str("\n");
    let mut block: u64 = 0;
    let mut object: u64 = 0;
    let mut glance: u64 = 0;

    let mut block_size: u32 = 0;
    let mut object_size: u32 = 0;
    let mut glance_size: u32 = 0;

    for pool in cluster_info.get_pool_info() {
        let name = pool.get_name();
        if name.contains("rbd") || name.contains("cinder") {
            // add this to block
            block += pool.get_num_kb();
            block_size = pool.get_replication_factor();
        }
        if name.contains("glance") {
            // add this to glance
            glance += pool.get_num_kb();
            glance_size = pool.get_replication_factor();
        }
        if name.contains("rgw") || name.contains("users") {
            // add this to object
            object += pool.get_num_kb();
            object_size = pool.get_replication_factor();
        }
    }

    buff.push_str(&format!(
        "{},{},{},{},{},{},{},{},{},{}",
        region,
        cluster_info.get_kb(),
        cluster_info.get_kb_avail(),
        cluster_info.get_kb_used(),
        block,
        object,
        glance,
        block_size,
        object_size,
        glance_size,
    ));
    buff
}

fn get_cli_args<'a>() -> ArgMatches<'a> {
    App::new("Ceph Usage Client")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Aggregate ceph cluster usage into csv format")
        .arg(
            Arg::with_name("host_list")
                .help(
                    "The file with ceph clusters to call.  1 server:port combination per line",
                )
                .long("hostlist")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("email_to")
                .help("Which user or users to email this information to")
                .long("emailto")
                .multiple(true)
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("email_from")
                .long("emailfrom")
                .help("The sending user")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("smtp")
                .long("smtpserver")
                .help("Which user or users to email this information to")
                .required(true)
                .takes_value(true),
        )
        .arg(Arg::with_name("v").short("v").multiple(true).help(
            "Sets the level of verbosity",
        ))
        .get_matches()
}

fn get_host_list(host_file: &Path) -> Result<Vec<(String, String)>> {
    let mut v: Vec<(String, String)> = Vec::new();
    let mut f = File::open(host_file)?;
    let mut buff = String::new();
    f.read_to_string(&mut buff)?;

    for l in buff.lines() {
        let parts: Vec<&str> = l.split(":").collect();
        if parts.len() != 2 {
            error!(
                "Format of host list must be host:port.  Unknown value: {}. Skipping",
                l
            );
            continue;
        }
        v.push((parts[0].to_string(), parts[1].trim().to_string()));
    }
    Ok(v)
}

fn get_cluster_usage(s: &mut Socket) -> Result<ClusterUsage> {
    debug!("Sending hello");
    let _ = s.send("Hello".as_bytes(), 0).map_err(|e| {
        Error::new(ErrorKind::Other, e)
    });
    let msg = s.recv_bytes(0).map_err(|e| Error::new(ErrorKind::Other, e))?;
    debug!("Got msg len: {}", msg.len());
    trace!("Parsing msg {:?} as hex", msg);
    let usage = parse_from_bytes::<api::service::ClusterUsage>(&msg)
        .map_err(|e| Error::new(ErrorKind::Other, e))?;
    Ok(usage)
}

fn main() {
    let matches = get_cli_args();
    let level = match matches.occurrences_of("v") {
        0 => log::LogLevelFilter::Info, //default
        1 => log::LogLevelFilter::Debug,
        _ => log::LogLevelFilter::Trace,
    };
    let email_to: Vec<&str> = matches.values_of("email_to").unwrap().collect();
    let email_from = matches.value_of("email_from").unwrap();
    let smtp_server = matches.value_of("smtp").unwrap();
    let hostlist = matches.value_of("host_list").unwrap();
    let _ = SimpleLogger::init(level, Config::default());
    info!("Starting up");
    let host_list = match get_host_list(&Path::new(hostlist)) {
        Ok(h) => h,
        Err(e) => {
            error!("Error getting host list: {:?}", e);
            return;
        }
    };
    for host in host_list {
        let mut s = match connect(&host.0, &host.1) {
            Ok(s) => s,
            Err(e) => {
                error!("Error connecting to socket: {:?}. Skipping host", e);
                continue;
            }
        };
        let usage = match get_cluster_usage(&mut s) {
            Ok(u) => u,
            Err(e) => {
                error!(
                    "Error getting cluster usage from {} {:?}. Skipping",
                    host.0,
                    e
                );
                continue;
            }
        };
        trace!("cluster_usage: {:#?}", usage);
        let csv = transform_csv(&usage, &host.0);
        trace!("csv: {}", csv);
        let email = match build_email(&email_to, email_from, &csv) {
            Ok(e) => e,
            Err(e) => {
                error!("Error building email: {:?}.  Skipping host", e);
                continue;
            }
        };
        match send_email(smtp_server, email) {
            Ok(_) => {
                info!("Email sent");
            }
            Err(e) => {
                error!("Email failed to send: {:?}. Skipping host", e);
                continue;
            }
        };
    }
}
