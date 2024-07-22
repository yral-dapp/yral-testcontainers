pub mod metadata {
    use testcontainers::{
        core::{ContainerPort, WaitFor},
        Image,
    };

    include!(concat!(env!("OUT_DIR"), "/meta/yral_metadata.rs"));

    pub const REST_PORT: ContainerPort = ContainerPort::Tcp(8001);
    pub const GRPC_PORT_TCP: ContainerPort = ContainerPort::Udp(8000);
    pub const GRPC_PORT_UDP: ContainerPort = ContainerPort::Udp(8000);

    pub struct YralMetadata;

    impl Image for YralMetadata {
        fn name(&self) -> &str {
            NAME
        }

        fn tag(&self) -> &str {
            TAG
        }

        fn ready_conditions(&self) -> Vec<WaitFor> {
            vec![WaitFor::millis(1000)]
        }

        fn expose_ports(&self) -> &[ContainerPort] {
            &[REST_PORT, GRPC_PORT_TCP, GRPC_PORT_UDP]
        }
    }
}

pub mod backend {
    use testcontainers::{
        core::{ContainerPort, WaitFor},
        Image,
    };

    include!(concat!(env!("OUT_DIR"), "/meta/yral_backend.rs"));

    pub const AGENT_PORT: ContainerPort = ContainerPort::Tcp(4943);
    pub const ADMIN_SECP_BYTES: [u8; 32] = [9, 64, 7, 55, 201, 208, 139, 219, 167, 201, 176, 6, 31, 109, 44, 248, 27, 241, 239, 56, 98, 100, 158, 36, 79, 233, 172, 151, 228, 187, 8, 224];

    pub struct YralBackend;

    impl Image for YralBackend {
        fn name(&self) -> &str {
            NAME
        }

        fn tag(&self) -> &str {
            TAG
        }

        fn ready_conditions(&self) -> Vec<WaitFor> {
            vec![WaitFor::millis(1500)]
        }

        fn expose_ports(&self) -> &[ContainerPort] {
            &[AGENT_PORT]
        }
    }
}
