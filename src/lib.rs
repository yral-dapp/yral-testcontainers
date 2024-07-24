pub mod metadata {
    use testcontainers::{
        core::{ContainerPort, WaitFor},
        Image,
    };

    pub const REST_PORT: ContainerPort = ContainerPort::Tcp(8001);
    pub const GRPC_PORT_TCP: ContainerPort = ContainerPort::Udp(8000);
    pub const GRPC_PORT_UDP: ContainerPort = ContainerPort::Udp(8000);

    pub struct YralMetadata {
        tag: String,
    }

    impl YralMetadata {
        pub fn new(tag: String) -> Self {
            Self { tag }
        }
    }

    impl Image for YralMetadata {
        fn name(&self) -> &str {
            "ghcr.io/go-bazzinga/yral-metadata-dev"
        }

        fn tag(&self) -> &str {
            &self.tag
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
        core::{wait::HttpWaitStrategy, ContainerPort, WaitFor},
        Image,
    };

    pub const AGENT_PORT: ContainerPort = ContainerPort::Tcp(4943);
    pub const ADMIN_SECP_BYTES: [u8; 32] = [
        9, 64, 7, 55, 201, 208, 139, 219, 167, 201, 176, 6, 31, 109, 44, 248, 27, 241, 239, 56, 98,
        100, 158, 36, 79, 233, 172, 151, 228, 187, 8, 224,
    ];

    pub struct YralBackend {
        tag: String,
    }

    impl YralBackend {
        pub fn new(tag: String) -> Self {
            Self { tag }
        }
    }

    impl Image for YralBackend {
        fn name(&self) -> &str {
            "ghcr.io/go-bazzinga/yral-backend-dev"
        }

        fn tag(&self) -> &str {
            &self.tag
        }

        fn ready_conditions(&self) -> Vec<WaitFor> {
            let wait_condition =
                HttpWaitStrategy::new("/api/v2/status").with_expected_status_code(200u16);

            vec![WaitFor::millis(1500), WaitFor::http(wait_condition)]
        }

        fn expose_ports(&self) -> &[ContainerPort] {
            &[AGENT_PORT]
        }
    }
}
