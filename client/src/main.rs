extern crate api;
#[macro_use]
extern crate clap;
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
use protobuf::parse_from_bytes;
use simplelog::{Config, SimpleLogger};
use zmq::Socket;
use zmq::Result as ZmqResult;

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

fn print_csv(cluster_info: &ClusterUsage, region: &str) {
    println!("region,total_kb,available_kb,used_kb,block_kb,object_kb,glance_kb");
    let mut block: u64 = 0;
    let mut object: u64 = 0;
    let mut glance: u64 = 0;

    for pool in cluster_info.get_pool_info() {
        let name = pool.get_name();
        if name.contains("rbd") || name.contains("cinder") {
            // add this to block
            block += pool.get_num_kb();
        }
        if name.contains("glance") {
            // add this to glance
            glance += pool.get_num_kb();
        }
        if name.contains("rgw") || name.contains("users") {
            // add this to object
            object += pool.get_num_kb();
        }
    }

    println!(
        "{},{},{},{},{},{},{}",
        region,
        cluster_info.get_kb(),
        cluster_info.get_kb_avail(),
        cluster_info.get_kb_used(),
        block,
        object,
        glance
    );
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
                error!("Error connecting to socket: {:?}", e);
                return;
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
        print_csv(&usage, &host.0);
    }
}
