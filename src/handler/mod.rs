mod auth;
mod user;
mod saldo;
mod topup;
mod transfer;
mod withdraw;

use self::auth::{get_user, login_user_handler, register_user_handler};
use self::user::{
    get_users,
    get_user as get_user_,
    create_user,
    update_user,
    delete_user
};
use self::saldo::{
    get_saldos,
    get_saldo,
    get_saldo_users,
    get_saldo_user,
    create_saldo,
    update_saldo,
    delete_saldo

};

use self::topup::{
    get_topups,
    get_topup,
    get_topup_users,
    get_topup_user,
    create_topup,
    update_topup,
    delete_topup
};


use self::transfer::{
    get_transfers,
    get_transfer,
    get_transfer_users,
    get_transfer_user,
    create_transfer,
    update_transfer,
    delete_transfer
};


use self::withdraw::{
    get_withdraws,
    get_withdraw,
    get_withdraw_users,
    get_withdraw_user,
    create_withdraw,
    update_withdraw,
    delete_withdraw
};

use actix_web::web;

pub fn router_config(conf: &mut web::ServiceConfig) {
    let router = web::scope("/api")
        // Auth routes
        .service(register_user_handler)
        .service(login_user_handler)
        .service(get_user)

        // User routes
        .service(get_users)
        .service(get_user_)
        .service(create_user)
        .service(update_user)
        .service(delete_user)

        // Saldo routes
        .service(get_saldos)
        .service(get_saldo)
        .service(get_saldo_users)
        .service(get_saldo_user)
        .service(create_saldo)
        .service(update_saldo)
        .service(delete_saldo)

        // Topup routes
        .service(get_topups)
        .service(get_topup)
        .service(get_topup_users)
        .service(get_topup_user)
        .service(create_topup)
        .service(update_topup)
        .service(delete_topup)

        // Transfer routes
        .service(get_transfers)
        .service(get_transfer)
        .service(get_transfer_users)
        .service(get_transfer_user)
        .service(create_transfer)
        .service(update_transfer)
        .service(delete_transfer)

        // Withdraw routes
        .service(get_withdraws)
        .service(get_withdraw)
        .service(get_withdraw_users)
        .service(get_withdraw_user)
        .service(create_withdraw)
        .service(update_withdraw)
        .service(delete_withdraw);

    conf.service(router);
}