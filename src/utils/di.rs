use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::{abstract_trait::{auth::DynAuthService, saldo::{DynSaldoRepository, DynSaldoService}, topup::{DynTopupRepository, DynTopupService}, transfer::{DynTransferRepository, DynTransferService}, user::{DynUserRepository, DynUserService}, withdraw::DynWithdrawService}, config::{hashing::Hashing, jwt_config::JwtConfig}, repository::{saldo::SaldoRepository, topup::TopupRepository, transfer::TransferRepository, user::UserRepository, withdraw::WithdrawRepository}, services::{auth::AuthService, saldo::SaldoService, topup::TopupService, transfer::TransferService, user::UserService, withdraw::WithdrawService}};



#[derive(Clone)]
pub struct DependenciesInject{
    pub auth_service: DynAuthService,
    pub user_service: DynUserService,
    pub saldo_service: DynSaldoService,
    pub topup_service: DynTopupService,
    pub transfer_service: DynTransferService,
    pub withdraw_service: DynWithdrawService,
}

impl DependenciesInject{
    pub fn new(pool: DatabaseConnection, hashing: Hashing, jwt_config: JwtConfig) -> Self{
        let user_repository = Arc::new(UserRepository::new(pool.clone())) as DynUserRepository;

        let user_service = Arc::new(UserService::new(user_repository.clone(), hashing.clone())) as DynUserService;

        let auth_service = Arc::new(AuthService::new(user_repository.clone(), hashing, jwt_config));


        let saldo_repository = Arc::new(SaldoRepository::new(pool.clone())) as DynSaldoRepository;

        let topup_repository = Arc::new(TopupRepository::new(pool.clone())) as DynTopupRepository;

        let transfer_repository = Arc::new(TransferRepository::new(pool.clone())) as DynTransferRepository;

        let withdraw_repository = Arc::new(WithdrawRepository::new(pool.clone()));


        let saldo_service = Arc::new(SaldoService::new(user_repository.clone(), saldo_repository.clone())) as DynSaldoService;

        let topup_service = Arc::new(TopupService::new(topup_repository.clone(), saldo_repository.clone(), user_repository.clone())) as DynTopupService;

        let transfer_service = Arc::new(TransferService::new(transfer_repository.clone(), saldo_repository.clone(), user_repository.clone())) as DynTransferService;

        let withdraw_service = Arc::new(WithdrawService::new(withdraw_repository.clone(), saldo_repository.clone(), user_repository.clone())) as DynWithdrawService;

        



        Self { auth_service, user_service, saldo_service, topup_service, transfer_service, withdraw_service }
    }

}