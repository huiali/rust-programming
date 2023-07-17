use crate::constants::serialize_object_id;
use actix_web::web::Data;
use anyhow::Result;
use bson::Bson;
use chrono::{FixedOffset, Utc};
use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use mongodb::bson::{self, doc};
use mongodb::Client;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

const MONGO_DB: &'static str = "samples";
const MONGO_COLL_DATASOURCE: &'static str = "sample";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sample {
    //标识
    #[serde(serialize_with = "serialize_object_id")]
    pub _id: Option<ObjectId>,
    pub _rid: String,
    //名称
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<Kv>>,
    //创建人
    pub created_by: Option<String>,
    //创建时间
    pub created_at: Option<DateTime>,
    //修改人
    pub updated_by: Option<String>,
    //修改时间
    pub updated_at: Option<DateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Kv {
    pub key: String,
    pub value: Bson,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
}

impl Sample {
    pub async fn find_all(
        data: Data<Mutex<Client>>,
        rid: &String,
        user_id: &String,
    ) -> Result<Vec<Sample>> {
        let collection = data
            .lock()
            .unwrap()
            .database(MONGO_DB)
            .collection::<Sample>(MONGO_COLL_DATASOURCE);

        let filter = doc! {"_rid":rid,"created_by":&user_id};
        let mut cursor = collection.find(filter.clone(), None).await?;
        let mut sample_arr = Vec::new();
        while let Some(sample) = cursor.try_next().await? {
            sample_arr.push(sample);
        }
        Ok(sample_arr)
    }

    pub async fn create(
        data: Data<Mutex<Client>>,
        req: Sample,
        user_id: String,
    ) -> Result<Option<Sample>> {
        let collection = data
            .lock()
            .unwrap()
            .database(MONGO_DB)
            .collection(MONGO_COLL_DATASOURCE);

        //创建人、创建时间、修改人、修改时间
        let chrono_dt = Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap());
        let bson_dt = bson::DateTime::from_chrono(chrono_dt);
        let mut sample = Sample {
            name: req.name,
            created_by: Some(user_id.clone()),
            created_at: Some(bson_dt),
            updated_by: Some(user_id.clone()),
            updated_at: Some(bson_dt),
            _id: None,
            _rid: req._rid,
            values: req.values,
        };

        let mut document = bson::to_document(&sample)?;
        document.remove("_id");
        let result = collection.insert_one(document, None).await?;
        sample._id = result.inserted_id.as_object_id();
        Ok(Some(sample))
    }

    pub async fn update(
        data: Data<Mutex<Client>>,
        sample: Sample,
        id: &String,
        user_id: &String,
    ) -> Result<Option<Sample>> {
        let collection = data
            .lock()
            .unwrap()
            .database(MONGO_DB)
            .collection::<Sample>(MONGO_COLL_DATASOURCE);

        let oid = ObjectId::parse_str(id)?;

        let chrono_dt = Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap());
        let bson_dt = bson::DateTime::from_chrono(chrono_dt);
        let mut document = bson::to_document(&sample)?;
        document.remove("_id");
        document.remove("_rid");
        document.remove("created_by");
        document.remove("created_at");
        document.insert("updated_by", user_id.clone());
        document.insert("updated_at", bson_dt);

        let query = doc! { "_id": oid};
        let update = doc! { "$set": document};

        collection.update_one(query, update, None).await?;
        return Ok(Some(sample));
    }

    pub async fn delete(data: Data<Mutex<Client>>, ids: Vec<ObjectId>) -> Result<Option<bool>> {
        let collection = data
            .lock()
            .unwrap()
            .database(MONGO_DB)
            .collection::<Sample>(MONGO_COLL_DATASOURCE);

        let query = doc! { "_id":{ "$in" : ids}};
        collection.delete_many(query, None).await?;
        return Ok(Some(true));
    }
}
