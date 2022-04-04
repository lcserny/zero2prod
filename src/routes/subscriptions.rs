use actix_web::{
    web::{self, Form},
    HttpResponse,
};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(form: Form<FormData>, connection_pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();

    let request_span = tracing::info_span!("Adding a new subscriber",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );

    let _request_span_guard = request_span.enter();
    let query_span = tracing::info_span!("New subscriber details have been saved");

    tracing::info!("Saving new subscriber details in the database");
    match sqlx::query!(
        r#"
        insert into subscriptions (id, email, name, subscribed_at) 
        values ($1, $2, $3, $4)
        "#,
        uuid::Uuid::new_v4(),
        form.email,
        form.name,
        chrono::Utc::now()
    )
    .execute(connection_pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
