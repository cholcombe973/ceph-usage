extern crate api;
extern crate ceph_usage;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate protobuf;
extern crate simplelog;
extern crate zmq;

use std::path::Path;
use std::thread;
use std::time::Duration;

use api::service::{ClusterUsage, PoolUsage};
use clap::{Arg, App};
use ceph_usage::*;
use protobuf::Message as ProtobufMsg;
use protobuf::RepeatedField;
use simplelog::{Config, SimpleLogger};
use zmq::{Message, Socket};

fn respond(s: &mut Socket, info: UsageInfo) -> Result<(), String> {
    let mut cluster_usage = ClusterUsage::new();
    cluster_usage.set_kb(info.cluster_usage.kb);
    cluster_usage.set_kb_used(info.cluster_usage.kb_used);
    cluster_usage.set_kb_avail(info.cluster_usage.kb_avail);
    cluster_usage.set_num_objects(info.cluster_usage.num_objects);
    let mut pool_info: Vec<PoolUsage> = Vec::new();
    for p in info.pool_usage {
        let mut encoded = PoolUsage::new();
        encoded.set_name(p.name);
        encoded.set_num_bytes(p.usage.num_bytes);
        encoded.set_num_kb(p.usage.num_kb);
        encoded.set_num_objects(p.usage.num_objects);
        encoded.set_num_object_clones(p.usage.num_object_clones);
        encoded.set_num_object_copies(p.usage.num_object_copies);
        encoded.set_num_objects_missing_on_primary(p.usage.num_objects_missing_on_primary);
        encoded.set_num_objects_unfound(p.usage.num_objects_unfound);
        encoded.set_num_objects_degraded(p.usage.num_objects_degraded);
        encoded.set_num_rd(p.usage.num_rd);
        encoded.set_num_rd_kb(p.usage.num_rd_kb);
        encoded.set_num_wr(p.usage.num_wr);
        encoded.set_num_wr_kb(p.usage.num_wr_kb);
        encoded.set_replication_factor(p.pool_size);
        pool_info.push(encoded);
    }

    cluster_usage.set_pool_info(RepeatedField::from_vec(pool_info));
    let encoded = cluster_usage.write_to_bytes().map_err(|e| e.to_string())?;
    let msg = Message::from_slice(&encoded).map_err(|e| e.to_string())?;
    debug!("Responding to client with msg len: {}", msg.len());
    s.send_msg(msg, 0).map_err(|e| e.to_string())?;
    Ok(())
}

fn listen(config_file: &Path, user: &str, port: &str) -> Result<(), String> {
    debug!("Starting zmq listener with version({:?})", zmq::version());
    let context = zmq::Context::new();
    let mut responder = context.socket(zmq::REP).map_err(|e| e.to_string())?;

    assert!(responder.bind(&format!("tcp://*:{}", port)).is_ok());

    loop {
        let _ = responder.recv_bytes(0).map_err(|e| e.to_string())?;
        debug!("Got connection");
        let usage = get_cluster_usage(user, &config_file)?;
        debug!("Sending response");
        respond(&mut responder, usage)?;
        thread::sleep(Duration::from_millis(10));
    }
}

fn main() {
    let _ = SimpleLogger::init(log::LogLevelFilter::Trace, Config::default());
    let matches = App::new("Disk usage")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Gather ceph usage information")
        .arg(
            Arg::with_name("ceph_config")
                .default_value("/etc/ceph/ceph.conf")
                .help("The ceph config file location")
                .long("cephconfig")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("ceph_user")
                .default_value("admin")
                .help("The user to connect to the cluster with")
                .long("cephuser")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("port")
                .default_value("8888")
                .help("Listen port")
                .long("port")
                .takes_value(true)
                .required(false),
        )
        .arg(Arg::with_name("v").short("v").multiple(true).help(
            "Sets the level of verbosity",
        ))
        .get_matches();
    let level = match matches.occurrences_of("v") {
        0 => log::LogLevelFilter::Info, //default
        1 => log::LogLevelFilter::Debug,
        _ => log::LogLevelFilter::Trace,
    };

    let _ = SimpleLogger::init(level, Config::default());
    let port = matches.value_of("port").unwrap();
    let ceph_user = matches.value_of("ceph_user").unwrap();
    let ceph_file = matches.value_of("ceph_config").unwrap();

    match listen(&Path::new(ceph_file), ceph_user, port) {
        Ok(_) => {
            println!("Done");
        }
        Err(e) => {
            println!("Listen failed: {:?}", e);
        }
    };
}
