use windows::Win32::{
    Foundation::{ERROR_BUFFER_OVERFLOW, ERROR_SUCCESS},
    NetworkManagement::IpHelper::{
        GetAdaptersAddresses, GAA_FLAG_INCLUDE_PREFIX, IP_ADAPTER_ADDRESSES_LH,
    },
    Networking::WinSock::AF_INET,
};

fn main() {
    // first call
    let family = AF_INET.0 as u32;
    let mut buflen = 0u32;
    let mut rc =
        unsafe { GetAdaptersAddresses(family, GAA_FLAG_INCLUDE_PREFIX, None, None, &mut buflen) };

    // second with the actual buffer size large enough to hold data
    if rc == ERROR_BUFFER_OVERFLOW.0 {
        let mut addr = vec![0u8; buflen as usize];
        let ptr = addr.as_mut_ptr() as *mut IP_ADAPTER_ADDRESSES_LH;

        rc = unsafe {
            GetAdaptersAddresses(
                family,
                GAA_FLAG_INCLUDE_PREFIX,
                None,
                Some(ptr),
                &mut buflen,
            )
        };

        // second with the actual buffer size large enough to hold data
        if rc == ERROR_SUCCESS.0 {
            // loop through adapters and grab DNS addresses
            let mut p = ptr;

            while !p.is_null() {
                unsafe {
                    let mut p_dns = (*p).FirstDnsServerAddress;

                    // loop through DNS addresses for this adapter
                    while !p_dns.is_null() {
                        let sockaddr = (*p_dns).Address.lpSockaddr;
                        println!(
                            "found DNS server: {:?} for adapter '{}'",
                            (*sockaddr).sa_data,
                            (*p).Description.display()
                        );

                        p_dns = (*p_dns).Next;
                    }

                    p = (*p).Next;
                }
            }
        } else {
            println!("error {} calling GetAdaptersAddresses()", rc);
        }
    } else {
        println!("error {} calling GetAdaptersAddresses()", rc);
    }
}
