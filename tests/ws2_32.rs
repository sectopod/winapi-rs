// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
#![cfg(windows)]
extern crate ws2_32;
use ws2_32::*;
#[inline(never)] fn bb<T>(_: T) {}
#[test] #[cfg(target_arch="x86")]
fn functions_x86() {
    bb(WSCInstallProviderAndChains);
}
#[test] #[cfg(target_arch="x86_64")]
fn functions_x86_64() {
    bb(WSCDeinstallProvider32);
    bb(WSCEnableNSProvider32);
    bb(WSCEnumNameSpaceProviders32);
    bb(WSCEnumNameSpaceProvidersEx32);
    bb(WSCEnumProtocols32);
    bb(WSCGetProviderInfo32);
    bb(WSCGetProviderPath32);
    bb(WSCInstallNameSpace32);
    bb(WSCInstallProvider64_32);
    bb(WSCInstallProviderAndChains64_32);
    bb(WSCSetProviderInfo32);
    bb(WSCUnInstallNameSpace32);
    bb(WSCUpdateProvider32);
    bb(WSCWriteNameSpaceOrder32);
    bb(WSCWriteProviderOrder32);
}
#[test] #[cfg(target_env="msvc")]
fn functions_msvc() {
    bb(GetAddrInfoExCancel);
    bb(GetAddrInfoExOverlappedResult);
    bb(GetHostNameW);
}
#[test]
fn functions() {
    bb(FreeAddrInfoEx);
    bb(FreeAddrInfoExW);
    bb(FreeAddrInfoW);
    bb(GetAddrInfoExA);
    bb(GetAddrInfoExW);
    bb(GetAddrInfoW);
    bb(GetNameInfoW);
    bb(InetNtopW);
    bb(InetPtonW);
    bb(SetAddrInfoExA);
    bb(SetAddrInfoExW);
    bb(WPUCompleteOverlappedRequest);
    bb(WSAAccept);
    bb(WSAAddressToStringA);
    bb(WSAAddressToStringW);
    bb(WSAAdvertiseProvider);
    bb(WSAAsyncGetHostByAddr);
    bb(WSAAsyncGetHostByName);
    bb(WSAAsyncGetProtoByName);
    bb(WSAAsyncGetProtoByNumber);
    bb(WSAAsyncGetServByName);
    bb(WSAAsyncGetServByPort);
    bb(WSAAsyncSelect);
    bb(WSACancelAsyncRequest);
    bb(WSACancelBlockingCall);
    bb(WSACleanup);
    bb(WSACloseEvent);
    bb(WSAConnect);
    bb(WSAConnectByList);
    bb(WSAConnectByNameA);
    bb(WSAConnectByNameW);
    bb(WSACreateEvent);
    bb(WSADuplicateSocketA);
    bb(WSADuplicateSocketW);
    bb(WSAEnumNameSpaceProvidersA);
    bb(WSAEnumNameSpaceProvidersExA);
    bb(WSAEnumNameSpaceProvidersExW);
    bb(WSAEnumNameSpaceProvidersW);
    bb(WSAEnumNetworkEvents);
    bb(WSAEnumProtocolsA);
    bb(WSAEnumProtocolsW);
    bb(WSAEventSelect);
    bb(WSAGetLastError);
    bb(WSAGetOverlappedResult);
    bb(WSAGetQOSByName);
    bb(WSAGetServiceClassInfoA);
    bb(WSAGetServiceClassInfoW);
    bb(WSAGetServiceClassNameByClassIdA);
    bb(WSAGetServiceClassNameByClassIdW);
    bb(WSAHtonl);
    bb(WSAHtons);
    bb(WSAIoctl);
    bb(WSAIsBlocking);
    bb(WSAJoinLeaf);
    bb(WSALookupServiceBeginA);
    bb(WSALookupServiceBeginW);
    bb(WSALookupServiceEnd);
    bb(WSALookupServiceNextA);
    bb(WSALookupServiceNextW);
    bb(WSANSPIoctl);
    bb(WSANtohl);
    bb(WSANtohs);
    bb(WSAPoll);
    bb(WSAProviderCompleteAsyncCall);
    bb(WSAProviderConfigChange);
    bb(WSARecv);
    bb(WSARecvDisconnect);
    bb(WSARecvFrom);
    bb(WSARemoveServiceClass);
    bb(WSAResetEvent);
    bb(WSASend);
    bb(WSASendDisconnect);
    bb(WSASendMsg);
    bb(WSASendTo);
    bb(WSASetBlockingHook);
    bb(WSASetEvent);
    bb(WSASetLastError);
    bb(WSASetServiceA);
    bb(WSASetServiceW);
    bb(WSASocketA);
    bb(WSASocketW);
    bb(WSAStartup);
    bb(WSAStringToAddressA);
    bb(WSAStringToAddressW);
    bb(WSAUnadvertiseProvider);
    bb(WSAUnhookBlockingHook);
    bb(WSAWaitForMultipleEvents);
    bb(WSCDeinstallProvider);
    bb(WSCEnableNSProvider);
    bb(WSCEnumProtocols);
    bb(WSCGetApplicationCategory);
    bb(WSCGetProviderInfo);
    bb(WSCGetProviderPath);
    bb(WSCInstallNameSpace);
    bb(WSCInstallNameSpaceEx);
    bb(WSCInstallProvider);
    bb(WSCSetApplicationCategory);
    bb(WSCSetProviderInfo);
    bb(WSCUpdateProvider);
    bb(WSCWriteNameSpaceOrder);
    bb(WSCWriteProviderOrder);
    bb(accept);
    bb(closesocket);
    bb(connect);
    bb(freeaddrinfo);
    bb(getaddrinfo);
    bb(gethostbyaddr);
    bb(gethostbyname);
    bb(gethostname);
    bb(getnameinfo);
    bb(getpeername);
    bb(getprotobyname);
    bb(getprotobynumber);
    bb(getservbyname);
    bb(getservbyport);
    bb(getsockname);
    bb(getsockopt);
    bb(htonl);
    bb(htons);
    bb(inet_addr);
    bb(inet_ntoa);
    bb(inet_ntop);
    bb(inet_pton);
    bb(ioctlsocket);
    bb(listen);
    bb(ntohl);
    bb(ntohs);
    bb(recv);
    bb(recvfrom);
    bb(select);
    bb(send);
    bb(sendto);
    bb(setsockopt);
    bb(shutdown);
    bb(socket);
}
