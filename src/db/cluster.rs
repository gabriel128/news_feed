use crate::Result;

use cassandra_cpp::{Cluster, Session};

#[derive(Debug)]
pub struct CassandraDb {
    cluster: Cluster

}

impl CassandraDb {
    pub fn new_cluster() -> Result<CassandraDb> {
        let contact_points = "127.0.0.1";

        let mut cluster = Cluster::default();
        cluster.set_contact_points(contact_points).unwrap();
        cluster.set_load_balance_round_robin();

        Ok(CassandraDb { cluster })
    }

    pub async fn new_session(&mut self) -> Result<Session> {
        println!("{:?}", self);
       let session = self.cluster.connect_async().await?;

       Ok(session)
    }
}
