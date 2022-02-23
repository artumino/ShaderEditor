use std::{os::windows::raw::HANDLE, ffi::{CString, CStr, c_void}, ptr, mem, fs::File};
use detour::{static_detour, GenericDetour, Function, Error};
use winapi::{um::{winuser::GetActiveWindow, libloaderapi::{GetModuleHandleA, LoadLibraryA}, d3d11::{D3D11CreateDeviceAndSwapChain, D3D11_SDK_VERSION, ID3D11Device, ID3D11DeviceContext, ID3D11ClassLinkage, ID3D11VertexShader, ID3D11PixelShader}, winnt::DriverType, d3dcommon::D3D_DRIVER_TYPE_HARDWARE}, shared::{minwindef::{HINSTANCE__, HMODULE}, ntdef::{NULL64, HRESULT}, dxgi::{IDXGIAdapter, DXGI_SWAP_CHAIN_DESC, IDXGISwapChain, IDXGIDevice}, dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM, dxgitype::{DXGI_MODE_DESC, DXGI_SAMPLE_DESC, DXGI_RATIONAL, DXGI_USAGE_RENDER_TARGET_OUTPUT}}};

pub type CCreateVertexShaderFn = unsafe fn(*mut ID3D11Device, *const c_void, usize, *mut ID3D11ClassLinkage, *mut *mut ID3D11VertexShader) -> HRESULT;
pub type CCreatePixelShaderFn = unsafe fn(*mut ID3D11Device, *const c_void, usize, *mut ID3D11ClassLinkage, *mut *mut ID3D11PixelShader) -> HRESULT;
static_detour! {
    pub static CreateVertexDetour: unsafe fn(*mut ID3D11Device, *const c_void, usize, *mut ID3D11ClassLinkage, *mut *mut ID3D11VertexShader) -> HRESULT;
    pub static CreatePixelDetour: unsafe fn(*mut ID3D11Device, *const c_void, usize, *mut ID3D11ClassLinkage, *mut *mut ID3D11PixelShader) -> HRESULT;
}

pub fn detoured_create_vertex_shader(device: *mut ID3D11Device, byte_code: *const c_void, length: usize, linkage: *mut ID3D11ClassLinkage, shader: *mut *mut ID3D11VertexShader) -> HRESULT {
    print!("Vertex");
    return unsafe { CreateVertexDetour.call(device, byte_code, length, linkage, shader) };
}

pub fn detoured_create_pixel_shader(device: *mut ID3D11Device, byte_code: *const c_void, length: usize, linkage: *mut ID3D11ClassLinkage, shader: *mut *mut ID3D11PixelShader) -> HRESULT {
    print!("Pixel");
    return unsafe { CreatePixelDetour.call(device, byte_code, length, linkage, shader) };
}