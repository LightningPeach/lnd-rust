// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait WalletUnlocker {
    fn gen_seed(&self, o: ::grpc::RequestOptions, p: super::rpc::GenSeedRequest) -> ::grpc::SingleResponse<super::rpc::GenSeedResponse>;

    fn init_wallet(&self, o: ::grpc::RequestOptions, p: super::rpc::InitWalletRequest) -> ::grpc::SingleResponse<super::rpc::InitWalletResponse>;

    fn unlock_wallet(&self, o: ::grpc::RequestOptions, p: super::rpc::UnlockWalletRequest) -> ::grpc::SingleResponse<super::rpc::UnlockWalletResponse>;

    fn change_password(&self, o: ::grpc::RequestOptions, p: super::rpc::ChangePasswordRequest) -> ::grpc::SingleResponse<super::rpc::ChangePasswordResponse>;
}

// client

pub struct WalletUnlockerClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_GenSeed: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::GenSeedRequest, super::rpc::GenSeedResponse>>,
    method_InitWallet: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::InitWalletRequest, super::rpc::InitWalletResponse>>,
    method_UnlockWallet: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::UnlockWalletRequest, super::rpc::UnlockWalletResponse>>,
    method_ChangePassword: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::ChangePasswordRequest, super::rpc::ChangePasswordResponse>>,
}

impl ::grpc::ClientStub for WalletUnlockerClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        WalletUnlockerClient {
            grpc_client: grpc_client,
            method_GenSeed: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.WalletUnlocker/GenSeed".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_InitWallet: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.WalletUnlocker/InitWallet".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_UnlockWallet: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.WalletUnlocker/UnlockWallet".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ChangePassword: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.WalletUnlocker/ChangePassword".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl WalletUnlocker for WalletUnlockerClient {
    fn gen_seed(&self, o: ::grpc::RequestOptions, p: super::rpc::GenSeedRequest) -> ::grpc::SingleResponse<super::rpc::GenSeedResponse> {
        self.grpc_client.call_unary(o, p, self.method_GenSeed.clone())
    }

    fn init_wallet(&self, o: ::grpc::RequestOptions, p: super::rpc::InitWalletRequest) -> ::grpc::SingleResponse<super::rpc::InitWalletResponse> {
        self.grpc_client.call_unary(o, p, self.method_InitWallet.clone())
    }

    fn unlock_wallet(&self, o: ::grpc::RequestOptions, p: super::rpc::UnlockWalletRequest) -> ::grpc::SingleResponse<super::rpc::UnlockWalletResponse> {
        self.grpc_client.call_unary(o, p, self.method_UnlockWallet.clone())
    }

    fn change_password(&self, o: ::grpc::RequestOptions, p: super::rpc::ChangePasswordRequest) -> ::grpc::SingleResponse<super::rpc::ChangePasswordResponse> {
        self.grpc_client.call_unary(o, p, self.method_ChangePassword.clone())
    }
}

// server

pub struct WalletUnlockerServer;


impl WalletUnlockerServer {
    pub fn new_service_def<H : WalletUnlocker + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/lnrpc.WalletUnlocker",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.WalletUnlocker/GenSeed".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.gen_seed(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.WalletUnlocker/InitWallet".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.init_wallet(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.WalletUnlocker/UnlockWallet".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.unlock_wallet(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.WalletUnlocker/ChangePassword".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.change_password(o, p))
                    },
                ),
            ],
        )
    }
}

// interface

pub trait Lightning {
    fn wallet_balance(&self, o: ::grpc::RequestOptions, p: super::rpc::WalletBalanceRequest) -> ::grpc::SingleResponse<super::rpc::WalletBalanceResponse>;

    fn channel_balance(&self, o: ::grpc::RequestOptions, p: super::rpc::ChannelBalanceRequest) -> ::grpc::SingleResponse<super::rpc::ChannelBalanceResponse>;

    fn get_transactions(&self, o: ::grpc::RequestOptions, p: super::rpc::GetTransactionsRequest) -> ::grpc::SingleResponse<super::rpc::TransactionDetails>;

    fn estimate_fee(&self, o: ::grpc::RequestOptions, p: super::rpc::EstimateFeeRequest) -> ::grpc::SingleResponse<super::rpc::EstimateFeeResponse>;

    fn send_coins(&self, o: ::grpc::RequestOptions, p: super::rpc::SendCoinsRequest) -> ::grpc::SingleResponse<super::rpc::SendCoinsResponse>;

    fn list_unspent(&self, o: ::grpc::RequestOptions, p: super::rpc::ListUnspentRequest) -> ::grpc::SingleResponse<super::rpc::ListUnspentResponse>;

    fn subscribe_transactions(&self, o: ::grpc::RequestOptions, p: super::rpc::GetTransactionsRequest) -> ::grpc::StreamingResponse<super::rpc::Transaction>;

    fn send_many(&self, o: ::grpc::RequestOptions, p: super::rpc::SendManyRequest) -> ::grpc::SingleResponse<super::rpc::SendManyResponse>;

    fn new_address(&self, o: ::grpc::RequestOptions, p: super::rpc::NewAddressRequest) -> ::grpc::SingleResponse<super::rpc::NewAddressResponse>;

    fn sign_message(&self, o: ::grpc::RequestOptions, p: super::rpc::SignMessageRequest) -> ::grpc::SingleResponse<super::rpc::SignMessageResponse>;

    fn verify_message(&self, o: ::grpc::RequestOptions, p: super::rpc::VerifyMessageRequest) -> ::grpc::SingleResponse<super::rpc::VerifyMessageResponse>;

    fn connect_peer(&self, o: ::grpc::RequestOptions, p: super::rpc::ConnectPeerRequest) -> ::grpc::SingleResponse<super::rpc::ConnectPeerResponse>;

    fn disconnect_peer(&self, o: ::grpc::RequestOptions, p: super::rpc::DisconnectPeerRequest) -> ::grpc::SingleResponse<super::rpc::DisconnectPeerResponse>;

    fn list_peers(&self, o: ::grpc::RequestOptions, p: super::rpc::ListPeersRequest) -> ::grpc::SingleResponse<super::rpc::ListPeersResponse>;

    fn get_info(&self, o: ::grpc::RequestOptions, p: super::rpc::GetInfoRequest) -> ::grpc::SingleResponse<super::rpc::GetInfoResponse>;

    fn pending_channels(&self, o: ::grpc::RequestOptions, p: super::rpc::PendingChannelsRequest) -> ::grpc::SingleResponse<super::rpc::PendingChannelsResponse>;

    fn list_channels(&self, o: ::grpc::RequestOptions, p: super::rpc::ListChannelsRequest) -> ::grpc::SingleResponse<super::rpc::ListChannelsResponse>;

    fn subscribe_channel_events(&self, o: ::grpc::RequestOptions, p: super::rpc::ChannelEventSubscription) -> ::grpc::StreamingResponse<super::rpc::ChannelEventUpdate>;

    fn closed_channels(&self, o: ::grpc::RequestOptions, p: super::rpc::ClosedChannelsRequest) -> ::grpc::SingleResponse<super::rpc::ClosedChannelsResponse>;

    fn open_channel_sync(&self, o: ::grpc::RequestOptions, p: super::rpc::OpenChannelRequest) -> ::grpc::SingleResponse<super::rpc::ChannelPoint>;

    fn open_channel(&self, o: ::grpc::RequestOptions, p: super::rpc::OpenChannelRequest) -> ::grpc::StreamingResponse<super::rpc::OpenStatusUpdate>;

    fn close_channel(&self, o: ::grpc::RequestOptions, p: super::rpc::CloseChannelRequest) -> ::grpc::StreamingResponse<super::rpc::CloseStatusUpdate>;

    fn abandon_channel(&self, o: ::grpc::RequestOptions, p: super::rpc::AbandonChannelRequest) -> ::grpc::SingleResponse<super::rpc::AbandonChannelResponse>;

    fn send_payment(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::rpc::SendRequest>) -> ::grpc::StreamingResponse<super::rpc::SendResponse>;

    fn send_payment_sync(&self, o: ::grpc::RequestOptions, p: super::rpc::SendRequest) -> ::grpc::SingleResponse<super::rpc::SendResponse>;

    fn send_to_route(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::rpc::SendToRouteRequest>) -> ::grpc::StreamingResponse<super::rpc::SendResponse>;

    fn send_to_route_sync(&self, o: ::grpc::RequestOptions, p: super::rpc::SendToRouteRequest) -> ::grpc::SingleResponse<super::rpc::SendResponse>;

    fn add_invoice(&self, o: ::grpc::RequestOptions, p: super::rpc::Invoice) -> ::grpc::SingleResponse<super::rpc::AddInvoiceResponse>;

    fn list_invoices(&self, o: ::grpc::RequestOptions, p: super::rpc::ListInvoiceRequest) -> ::grpc::SingleResponse<super::rpc::ListInvoiceResponse>;

    fn lookup_invoice(&self, o: ::grpc::RequestOptions, p: super::rpc::PaymentHash) -> ::grpc::SingleResponse<super::rpc::Invoice>;

    fn subscribe_invoices(&self, o: ::grpc::RequestOptions, p: super::rpc::InvoiceSubscription) -> ::grpc::StreamingResponse<super::rpc::Invoice>;

    fn decode_pay_req(&self, o: ::grpc::RequestOptions, p: super::rpc::PayReqString) -> ::grpc::SingleResponse<super::rpc::PayReq>;

    fn list_payments(&self, o: ::grpc::RequestOptions, p: super::rpc::ListPaymentsRequest) -> ::grpc::SingleResponse<super::rpc::ListPaymentsResponse>;

    fn delete_all_payments(&self, o: ::grpc::RequestOptions, p: super::rpc::DeleteAllPaymentsRequest) -> ::grpc::SingleResponse<super::rpc::DeleteAllPaymentsResponse>;

    fn describe_graph(&self, o: ::grpc::RequestOptions, p: super::rpc::ChannelGraphRequest) -> ::grpc::SingleResponse<super::rpc::ChannelGraph>;

    fn get_chan_info(&self, o: ::grpc::RequestOptions, p: super::rpc::ChanInfoRequest) -> ::grpc::SingleResponse<super::rpc::ChannelEdge>;

    fn get_node_info(&self, o: ::grpc::RequestOptions, p: super::rpc::NodeInfoRequest) -> ::grpc::SingleResponse<super::rpc::NodeInfo>;

    fn query_routes(&self, o: ::grpc::RequestOptions, p: super::rpc::QueryRoutesRequest) -> ::grpc::SingleResponse<super::rpc::QueryRoutesResponse>;

    fn get_network_info(&self, o: ::grpc::RequestOptions, p: super::rpc::NetworkInfoRequest) -> ::grpc::SingleResponse<super::rpc::NetworkInfo>;

    fn stop_daemon(&self, o: ::grpc::RequestOptions, p: super::rpc::StopRequest) -> ::grpc::SingleResponse<super::rpc::StopResponse>;

    fn subscribe_channel_graph(&self, o: ::grpc::RequestOptions, p: super::rpc::GraphTopologySubscription) -> ::grpc::StreamingResponse<super::rpc::GraphTopologyUpdate>;

    fn debug_level(&self, o: ::grpc::RequestOptions, p: super::rpc::DebugLevelRequest) -> ::grpc::SingleResponse<super::rpc::DebugLevelResponse>;

    fn fee_report(&self, o: ::grpc::RequestOptions, p: super::rpc::FeeReportRequest) -> ::grpc::SingleResponse<super::rpc::FeeReportResponse>;

    fn update_channel_policy(&self, o: ::grpc::RequestOptions, p: super::rpc::PolicyUpdateRequest) -> ::grpc::SingleResponse<super::rpc::PolicyUpdateResponse>;

    fn forwarding_history(&self, o: ::grpc::RequestOptions, p: super::rpc::ForwardingHistoryRequest) -> ::grpc::SingleResponse<super::rpc::ForwardingHistoryResponse>;

    fn export_channel_backup(&self, o: ::grpc::RequestOptions, p: super::rpc::ExportChannelBackupRequest) -> ::grpc::SingleResponse<super::rpc::ChannelBackup>;

    fn export_all_channel_backups(&self, o: ::grpc::RequestOptions, p: super::rpc::ChanBackupExportRequest) -> ::grpc::SingleResponse<super::rpc::ChanBackupSnapshot>;

    fn verify_chan_backup(&self, o: ::grpc::RequestOptions, p: super::rpc::ChanBackupSnapshot) -> ::grpc::SingleResponse<super::rpc::VerifyChanBackupResponse>;

    fn restore_channel_backups(&self, o: ::grpc::RequestOptions, p: super::rpc::RestoreChanBackupRequest) -> ::grpc::SingleResponse<super::rpc::RestoreBackupResponse>;

    fn subscribe_channel_backups(&self, o: ::grpc::RequestOptions, p: super::rpc::ChannelBackupSubscription) -> ::grpc::StreamingResponse<super::rpc::ChanBackupSnapshot>;
}

// client

pub struct LightningClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_WalletBalance: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::WalletBalanceRequest, super::rpc::WalletBalanceResponse>>,
    method_ChannelBalance: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::ChannelBalanceRequest, super::rpc::ChannelBalanceResponse>>,
    method_GetTransactions: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::GetTransactionsRequest, super::rpc::TransactionDetails>>,
    method_EstimateFee: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::EstimateFeeRequest, super::rpc::EstimateFeeResponse>>,
    method_SendCoins: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::SendCoinsRequest, super::rpc::SendCoinsResponse>>,
    method_ListUnspent: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::ListUnspentRequest, super::rpc::ListUnspentResponse>>,
    method_SubscribeTransactions: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::GetTransactionsRequest, super::rpc::Transaction>>,
    method_SendMany: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::SendManyRequest, super::rpc::SendManyResponse>>,
    method_NewAddress: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::NewAddressRequest, super::rpc::NewAddressResponse>>,
    method_SignMessage: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::SignMessageRequest, super::rpc::SignMessageResponse>>,
    method_VerifyMessage: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::VerifyMessageRequest, super::rpc::VerifyMessageResponse>>,
    method_ConnectPeer: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::ConnectPeerRequest, super::rpc::ConnectPeerResponse>>,
    method_DisconnectPeer: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::DisconnectPeerRequest, super::rpc::DisconnectPeerResponse>>,
    method_ListPeers: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::ListPeersRequest, super::rpc::ListPeersResponse>>,
    method_GetInfo: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::GetInfoRequest, super::rpc::GetInfoResponse>>,
    method_PendingChannels: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::PendingChannelsRequest, super::rpc::PendingChannelsResponse>>,
    method_ListChannels: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::ListChannelsRequest, super::rpc::ListChannelsResponse>>,
    method_SubscribeChannelEvents: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::ChannelEventSubscription, super::rpc::ChannelEventUpdate>>,
    method_ClosedChannels: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::ClosedChannelsRequest, super::rpc::ClosedChannelsResponse>>,
    method_OpenChannelSync: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::OpenChannelRequest, super::rpc::ChannelPoint>>,
    method_OpenChannel: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::OpenChannelRequest, super::rpc::OpenStatusUpdate>>,
    method_CloseChannel: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::CloseChannelRequest, super::rpc::CloseStatusUpdate>>,
    method_AbandonChannel: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::AbandonChannelRequest, super::rpc::AbandonChannelResponse>>,
    method_SendPayment: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::SendRequest, super::rpc::SendResponse>>,
    method_SendPaymentSync: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::SendRequest, super::rpc::SendResponse>>,
    method_SendToRoute: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::SendToRouteRequest, super::rpc::SendResponse>>,
    method_SendToRouteSync: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::SendToRouteRequest, super::rpc::SendResponse>>,
    method_AddInvoice: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::Invoice, super::rpc::AddInvoiceResponse>>,
    method_ListInvoices: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::ListInvoiceRequest, super::rpc::ListInvoiceResponse>>,
    method_LookupInvoice: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::PaymentHash, super::rpc::Invoice>>,
    method_SubscribeInvoices: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::InvoiceSubscription, super::rpc::Invoice>>,
    method_DecodePayReq: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::PayReqString, super::rpc::PayReq>>,
    method_ListPayments: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::ListPaymentsRequest, super::rpc::ListPaymentsResponse>>,
    method_DeleteAllPayments: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::DeleteAllPaymentsRequest, super::rpc::DeleteAllPaymentsResponse>>,
    method_DescribeGraph: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::ChannelGraphRequest, super::rpc::ChannelGraph>>,
    method_GetChanInfo: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::ChanInfoRequest, super::rpc::ChannelEdge>>,
    method_GetNodeInfo: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::NodeInfoRequest, super::rpc::NodeInfo>>,
    method_QueryRoutes: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::QueryRoutesRequest, super::rpc::QueryRoutesResponse>>,
    method_GetNetworkInfo: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::NetworkInfoRequest, super::rpc::NetworkInfo>>,
    method_StopDaemon: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::StopRequest, super::rpc::StopResponse>>,
    method_SubscribeChannelGraph: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::GraphTopologySubscription, super::rpc::GraphTopologyUpdate>>,
    method_DebugLevel: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::DebugLevelRequest, super::rpc::DebugLevelResponse>>,
    method_FeeReport: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::FeeReportRequest, super::rpc::FeeReportResponse>>,
    method_UpdateChannelPolicy: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::PolicyUpdateRequest, super::rpc::PolicyUpdateResponse>>,
    method_ForwardingHistory: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::ForwardingHistoryRequest, super::rpc::ForwardingHistoryResponse>>,
    method_ExportChannelBackup: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::ExportChannelBackupRequest, super::rpc::ChannelBackup>>,
    method_ExportAllChannelBackups: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::ChanBackupExportRequest, super::rpc::ChanBackupSnapshot>>,
    method_VerifyChanBackup: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::ChanBackupSnapshot, super::rpc::VerifyChanBackupResponse>>,
    method_RestoreChannelBackups: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::RestoreChanBackupRequest, super::rpc::RestoreBackupResponse>>,
    method_SubscribeChannelBackups: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rpc::ChannelBackupSubscription, super::rpc::ChanBackupSnapshot>>,
}

impl ::grpc::ClientStub for LightningClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        LightningClient {
            grpc_client: grpc_client,
            method_WalletBalance: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/WalletBalance".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ChannelBalance: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/ChannelBalance".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetTransactions: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/GetTransactions".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_EstimateFee: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/EstimateFee".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SendCoins: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/SendCoins".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ListUnspent: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/ListUnspent".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SubscribeTransactions: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/SubscribeTransactions".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SendMany: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/SendMany".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_NewAddress: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/NewAddress".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SignMessage: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/SignMessage".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_VerifyMessage: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/VerifyMessage".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ConnectPeer: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/ConnectPeer".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_DisconnectPeer: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/DisconnectPeer".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ListPeers: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/ListPeers".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetInfo: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/GetInfo".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_PendingChannels: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/PendingChannels".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ListChannels: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/ListChannels".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SubscribeChannelEvents: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/SubscribeChannelEvents".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ClosedChannels: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/ClosedChannels".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_OpenChannelSync: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/OpenChannelSync".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_OpenChannel: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/OpenChannel".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_CloseChannel: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/CloseChannel".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_AbandonChannel: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/AbandonChannel".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SendPayment: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/SendPayment".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Bidi,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SendPaymentSync: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/SendPaymentSync".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SendToRoute: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/SendToRoute".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Bidi,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SendToRouteSync: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/SendToRouteSync".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_AddInvoice: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/AddInvoice".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ListInvoices: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/ListInvoices".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_LookupInvoice: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/LookupInvoice".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SubscribeInvoices: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/SubscribeInvoices".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_DecodePayReq: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/DecodePayReq".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ListPayments: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/ListPayments".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_DeleteAllPayments: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/DeleteAllPayments".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_DescribeGraph: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/DescribeGraph".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetChanInfo: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/GetChanInfo".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetNodeInfo: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/GetNodeInfo".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_QueryRoutes: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/QueryRoutes".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetNetworkInfo: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/GetNetworkInfo".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_StopDaemon: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/StopDaemon".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SubscribeChannelGraph: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/SubscribeChannelGraph".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_DebugLevel: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/DebugLevel".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_FeeReport: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/FeeReport".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_UpdateChannelPolicy: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/UpdateChannelPolicy".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ForwardingHistory: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/ForwardingHistory".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ExportChannelBackup: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/ExportChannelBackup".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ExportAllChannelBackups: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/ExportAllChannelBackups".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_VerifyChanBackup: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/VerifyChanBackup".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_RestoreChannelBackups: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/RestoreChannelBackups".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SubscribeChannelBackups: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/lnrpc.Lightning/SubscribeChannelBackups".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl Lightning for LightningClient {
    fn wallet_balance(&self, o: ::grpc::RequestOptions, p: super::rpc::WalletBalanceRequest) -> ::grpc::SingleResponse<super::rpc::WalletBalanceResponse> {
        self.grpc_client.call_unary(o, p, self.method_WalletBalance.clone())
    }

    fn channel_balance(&self, o: ::grpc::RequestOptions, p: super::rpc::ChannelBalanceRequest) -> ::grpc::SingleResponse<super::rpc::ChannelBalanceResponse> {
        self.grpc_client.call_unary(o, p, self.method_ChannelBalance.clone())
    }

    fn get_transactions(&self, o: ::grpc::RequestOptions, p: super::rpc::GetTransactionsRequest) -> ::grpc::SingleResponse<super::rpc::TransactionDetails> {
        self.grpc_client.call_unary(o, p, self.method_GetTransactions.clone())
    }

    fn estimate_fee(&self, o: ::grpc::RequestOptions, p: super::rpc::EstimateFeeRequest) -> ::grpc::SingleResponse<super::rpc::EstimateFeeResponse> {
        self.grpc_client.call_unary(o, p, self.method_EstimateFee.clone())
    }

    fn send_coins(&self, o: ::grpc::RequestOptions, p: super::rpc::SendCoinsRequest) -> ::grpc::SingleResponse<super::rpc::SendCoinsResponse> {
        self.grpc_client.call_unary(o, p, self.method_SendCoins.clone())
    }

    fn list_unspent(&self, o: ::grpc::RequestOptions, p: super::rpc::ListUnspentRequest) -> ::grpc::SingleResponse<super::rpc::ListUnspentResponse> {
        self.grpc_client.call_unary(o, p, self.method_ListUnspent.clone())
    }

    fn subscribe_transactions(&self, o: ::grpc::RequestOptions, p: super::rpc::GetTransactionsRequest) -> ::grpc::StreamingResponse<super::rpc::Transaction> {
        self.grpc_client.call_server_streaming(o, p, self.method_SubscribeTransactions.clone())
    }

    fn send_many(&self, o: ::grpc::RequestOptions, p: super::rpc::SendManyRequest) -> ::grpc::SingleResponse<super::rpc::SendManyResponse> {
        self.grpc_client.call_unary(o, p, self.method_SendMany.clone())
    }

    fn new_address(&self, o: ::grpc::RequestOptions, p: super::rpc::NewAddressRequest) -> ::grpc::SingleResponse<super::rpc::NewAddressResponse> {
        self.grpc_client.call_unary(o, p, self.method_NewAddress.clone())
    }

    fn sign_message(&self, o: ::grpc::RequestOptions, p: super::rpc::SignMessageRequest) -> ::grpc::SingleResponse<super::rpc::SignMessageResponse> {
        self.grpc_client.call_unary(o, p, self.method_SignMessage.clone())
    }

    fn verify_message(&self, o: ::grpc::RequestOptions, p: super::rpc::VerifyMessageRequest) -> ::grpc::SingleResponse<super::rpc::VerifyMessageResponse> {
        self.grpc_client.call_unary(o, p, self.method_VerifyMessage.clone())
    }

    fn connect_peer(&self, o: ::grpc::RequestOptions, p: super::rpc::ConnectPeerRequest) -> ::grpc::SingleResponse<super::rpc::ConnectPeerResponse> {
        self.grpc_client.call_unary(o, p, self.method_ConnectPeer.clone())
    }

    fn disconnect_peer(&self, o: ::grpc::RequestOptions, p: super::rpc::DisconnectPeerRequest) -> ::grpc::SingleResponse<super::rpc::DisconnectPeerResponse> {
        self.grpc_client.call_unary(o, p, self.method_DisconnectPeer.clone())
    }

    fn list_peers(&self, o: ::grpc::RequestOptions, p: super::rpc::ListPeersRequest) -> ::grpc::SingleResponse<super::rpc::ListPeersResponse> {
        self.grpc_client.call_unary(o, p, self.method_ListPeers.clone())
    }

    fn get_info(&self, o: ::grpc::RequestOptions, p: super::rpc::GetInfoRequest) -> ::grpc::SingleResponse<super::rpc::GetInfoResponse> {
        self.grpc_client.call_unary(o, p, self.method_GetInfo.clone())
    }

    fn pending_channels(&self, o: ::grpc::RequestOptions, p: super::rpc::PendingChannelsRequest) -> ::grpc::SingleResponse<super::rpc::PendingChannelsResponse> {
        self.grpc_client.call_unary(o, p, self.method_PendingChannels.clone())
    }

    fn list_channels(&self, o: ::grpc::RequestOptions, p: super::rpc::ListChannelsRequest) -> ::grpc::SingleResponse<super::rpc::ListChannelsResponse> {
        self.grpc_client.call_unary(o, p, self.method_ListChannels.clone())
    }

    fn subscribe_channel_events(&self, o: ::grpc::RequestOptions, p: super::rpc::ChannelEventSubscription) -> ::grpc::StreamingResponse<super::rpc::ChannelEventUpdate> {
        self.grpc_client.call_server_streaming(o, p, self.method_SubscribeChannelEvents.clone())
    }

    fn closed_channels(&self, o: ::grpc::RequestOptions, p: super::rpc::ClosedChannelsRequest) -> ::grpc::SingleResponse<super::rpc::ClosedChannelsResponse> {
        self.grpc_client.call_unary(o, p, self.method_ClosedChannels.clone())
    }

    fn open_channel_sync(&self, o: ::grpc::RequestOptions, p: super::rpc::OpenChannelRequest) -> ::grpc::SingleResponse<super::rpc::ChannelPoint> {
        self.grpc_client.call_unary(o, p, self.method_OpenChannelSync.clone())
    }

    fn open_channel(&self, o: ::grpc::RequestOptions, p: super::rpc::OpenChannelRequest) -> ::grpc::StreamingResponse<super::rpc::OpenStatusUpdate> {
        self.grpc_client.call_server_streaming(o, p, self.method_OpenChannel.clone())
    }

    fn close_channel(&self, o: ::grpc::RequestOptions, p: super::rpc::CloseChannelRequest) -> ::grpc::StreamingResponse<super::rpc::CloseStatusUpdate> {
        self.grpc_client.call_server_streaming(o, p, self.method_CloseChannel.clone())
    }

    fn abandon_channel(&self, o: ::grpc::RequestOptions, p: super::rpc::AbandonChannelRequest) -> ::grpc::SingleResponse<super::rpc::AbandonChannelResponse> {
        self.grpc_client.call_unary(o, p, self.method_AbandonChannel.clone())
    }

    fn send_payment(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::rpc::SendRequest>) -> ::grpc::StreamingResponse<super::rpc::SendResponse> {
        self.grpc_client.call_bidi(o, p, self.method_SendPayment.clone())
    }

    fn send_payment_sync(&self, o: ::grpc::RequestOptions, p: super::rpc::SendRequest) -> ::grpc::SingleResponse<super::rpc::SendResponse> {
        self.grpc_client.call_unary(o, p, self.method_SendPaymentSync.clone())
    }

    fn send_to_route(&self, o: ::grpc::RequestOptions, p: ::grpc::StreamingRequest<super::rpc::SendToRouteRequest>) -> ::grpc::StreamingResponse<super::rpc::SendResponse> {
        self.grpc_client.call_bidi(o, p, self.method_SendToRoute.clone())
    }

    fn send_to_route_sync(&self, o: ::grpc::RequestOptions, p: super::rpc::SendToRouteRequest) -> ::grpc::SingleResponse<super::rpc::SendResponse> {
        self.grpc_client.call_unary(o, p, self.method_SendToRouteSync.clone())
    }

    fn add_invoice(&self, o: ::grpc::RequestOptions, p: super::rpc::Invoice) -> ::grpc::SingleResponse<super::rpc::AddInvoiceResponse> {
        self.grpc_client.call_unary(o, p, self.method_AddInvoice.clone())
    }

    fn list_invoices(&self, o: ::grpc::RequestOptions, p: super::rpc::ListInvoiceRequest) -> ::grpc::SingleResponse<super::rpc::ListInvoiceResponse> {
        self.grpc_client.call_unary(o, p, self.method_ListInvoices.clone())
    }

    fn lookup_invoice(&self, o: ::grpc::RequestOptions, p: super::rpc::PaymentHash) -> ::grpc::SingleResponse<super::rpc::Invoice> {
        self.grpc_client.call_unary(o, p, self.method_LookupInvoice.clone())
    }

    fn subscribe_invoices(&self, o: ::grpc::RequestOptions, p: super::rpc::InvoiceSubscription) -> ::grpc::StreamingResponse<super::rpc::Invoice> {
        self.grpc_client.call_server_streaming(o, p, self.method_SubscribeInvoices.clone())
    }

    fn decode_pay_req(&self, o: ::grpc::RequestOptions, p: super::rpc::PayReqString) -> ::grpc::SingleResponse<super::rpc::PayReq> {
        self.grpc_client.call_unary(o, p, self.method_DecodePayReq.clone())
    }

    fn list_payments(&self, o: ::grpc::RequestOptions, p: super::rpc::ListPaymentsRequest) -> ::grpc::SingleResponse<super::rpc::ListPaymentsResponse> {
        self.grpc_client.call_unary(o, p, self.method_ListPayments.clone())
    }

    fn delete_all_payments(&self, o: ::grpc::RequestOptions, p: super::rpc::DeleteAllPaymentsRequest) -> ::grpc::SingleResponse<super::rpc::DeleteAllPaymentsResponse> {
        self.grpc_client.call_unary(o, p, self.method_DeleteAllPayments.clone())
    }

    fn describe_graph(&self, o: ::grpc::RequestOptions, p: super::rpc::ChannelGraphRequest) -> ::grpc::SingleResponse<super::rpc::ChannelGraph> {
        self.grpc_client.call_unary(o, p, self.method_DescribeGraph.clone())
    }

    fn get_chan_info(&self, o: ::grpc::RequestOptions, p: super::rpc::ChanInfoRequest) -> ::grpc::SingleResponse<super::rpc::ChannelEdge> {
        self.grpc_client.call_unary(o, p, self.method_GetChanInfo.clone())
    }

    fn get_node_info(&self, o: ::grpc::RequestOptions, p: super::rpc::NodeInfoRequest) -> ::grpc::SingleResponse<super::rpc::NodeInfo> {
        self.grpc_client.call_unary(o, p, self.method_GetNodeInfo.clone())
    }

    fn query_routes(&self, o: ::grpc::RequestOptions, p: super::rpc::QueryRoutesRequest) -> ::grpc::SingleResponse<super::rpc::QueryRoutesResponse> {
        self.grpc_client.call_unary(o, p, self.method_QueryRoutes.clone())
    }

    fn get_network_info(&self, o: ::grpc::RequestOptions, p: super::rpc::NetworkInfoRequest) -> ::grpc::SingleResponse<super::rpc::NetworkInfo> {
        self.grpc_client.call_unary(o, p, self.method_GetNetworkInfo.clone())
    }

    fn stop_daemon(&self, o: ::grpc::RequestOptions, p: super::rpc::StopRequest) -> ::grpc::SingleResponse<super::rpc::StopResponse> {
        self.grpc_client.call_unary(o, p, self.method_StopDaemon.clone())
    }

    fn subscribe_channel_graph(&self, o: ::grpc::RequestOptions, p: super::rpc::GraphTopologySubscription) -> ::grpc::StreamingResponse<super::rpc::GraphTopologyUpdate> {
        self.grpc_client.call_server_streaming(o, p, self.method_SubscribeChannelGraph.clone())
    }

    fn debug_level(&self, o: ::grpc::RequestOptions, p: super::rpc::DebugLevelRequest) -> ::grpc::SingleResponse<super::rpc::DebugLevelResponse> {
        self.grpc_client.call_unary(o, p, self.method_DebugLevel.clone())
    }

    fn fee_report(&self, o: ::grpc::RequestOptions, p: super::rpc::FeeReportRequest) -> ::grpc::SingleResponse<super::rpc::FeeReportResponse> {
        self.grpc_client.call_unary(o, p, self.method_FeeReport.clone())
    }

    fn update_channel_policy(&self, o: ::grpc::RequestOptions, p: super::rpc::PolicyUpdateRequest) -> ::grpc::SingleResponse<super::rpc::PolicyUpdateResponse> {
        self.grpc_client.call_unary(o, p, self.method_UpdateChannelPolicy.clone())
    }

    fn forwarding_history(&self, o: ::grpc::RequestOptions, p: super::rpc::ForwardingHistoryRequest) -> ::grpc::SingleResponse<super::rpc::ForwardingHistoryResponse> {
        self.grpc_client.call_unary(o, p, self.method_ForwardingHistory.clone())
    }

    fn export_channel_backup(&self, o: ::grpc::RequestOptions, p: super::rpc::ExportChannelBackupRequest) -> ::grpc::SingleResponse<super::rpc::ChannelBackup> {
        self.grpc_client.call_unary(o, p, self.method_ExportChannelBackup.clone())
    }

    fn export_all_channel_backups(&self, o: ::grpc::RequestOptions, p: super::rpc::ChanBackupExportRequest) -> ::grpc::SingleResponse<super::rpc::ChanBackupSnapshot> {
        self.grpc_client.call_unary(o, p, self.method_ExportAllChannelBackups.clone())
    }

    fn verify_chan_backup(&self, o: ::grpc::RequestOptions, p: super::rpc::ChanBackupSnapshot) -> ::grpc::SingleResponse<super::rpc::VerifyChanBackupResponse> {
        self.grpc_client.call_unary(o, p, self.method_VerifyChanBackup.clone())
    }

    fn restore_channel_backups(&self, o: ::grpc::RequestOptions, p: super::rpc::RestoreChanBackupRequest) -> ::grpc::SingleResponse<super::rpc::RestoreBackupResponse> {
        self.grpc_client.call_unary(o, p, self.method_RestoreChannelBackups.clone())
    }

    fn subscribe_channel_backups(&self, o: ::grpc::RequestOptions, p: super::rpc::ChannelBackupSubscription) -> ::grpc::StreamingResponse<super::rpc::ChanBackupSnapshot> {
        self.grpc_client.call_server_streaming(o, p, self.method_SubscribeChannelBackups.clone())
    }
}

// server

pub struct LightningServer;


impl LightningServer {
    pub fn new_service_def<H : Lightning + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/lnrpc.Lightning",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/WalletBalance".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.wallet_balance(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/ChannelBalance".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.channel_balance(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/GetTransactions".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_transactions(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/EstimateFee".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.estimate_fee(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/SendCoins".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.send_coins(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/ListUnspent".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_unspent(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/SubscribeTransactions".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.subscribe_transactions(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/SendMany".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.send_many(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/NewAddress".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.new_address(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/SignMessage".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.sign_message(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/VerifyMessage".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.verify_message(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/ConnectPeer".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.connect_peer(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/DisconnectPeer".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.disconnect_peer(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/ListPeers".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_peers(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/GetInfo".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_info(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/PendingChannels".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.pending_channels(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/ListChannels".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_channels(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/SubscribeChannelEvents".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.subscribe_channel_events(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/ClosedChannels".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.closed_channels(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/OpenChannelSync".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.open_channel_sync(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/OpenChannel".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.open_channel(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/CloseChannel".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.close_channel(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/AbandonChannel".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.abandon_channel(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/SendPayment".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Bidi,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerBidi::new(move |o, p| handler_copy.send_payment(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/SendPaymentSync".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.send_payment_sync(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/SendToRoute".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Bidi,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerBidi::new(move |o, p| handler_copy.send_to_route(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/SendToRouteSync".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.send_to_route_sync(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/AddInvoice".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.add_invoice(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/ListInvoices".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_invoices(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/LookupInvoice".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.lookup_invoice(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/SubscribeInvoices".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.subscribe_invoices(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/DecodePayReq".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.decode_pay_req(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/ListPayments".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_payments(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/DeleteAllPayments".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.delete_all_payments(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/DescribeGraph".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.describe_graph(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/GetChanInfo".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_chan_info(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/GetNodeInfo".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_node_info(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/QueryRoutes".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.query_routes(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/GetNetworkInfo".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_network_info(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/StopDaemon".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.stop_daemon(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/SubscribeChannelGraph".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.subscribe_channel_graph(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/DebugLevel".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.debug_level(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/FeeReport".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.fee_report(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/UpdateChannelPolicy".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.update_channel_policy(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/ForwardingHistory".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.forwarding_history(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/ExportChannelBackup".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.export_channel_backup(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/ExportAllChannelBackups".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.export_all_channel_backups(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/VerifyChanBackup".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.verify_chan_backup(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/RestoreChannelBackups".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.restore_channel_backups(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/lnrpc.Lightning/SubscribeChannelBackups".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.subscribe_channel_backups(o, p))
                    },
                ),
            ],
        )
    }
}
