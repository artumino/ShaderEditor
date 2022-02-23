use std::{os::windows::raw::HANDLE, ffi::{CString, CStr, c_void}, ptr, mem, fs::{File, self}, path::Path};
use detour::{static_detour, GenericDetour, Function, Error};
use md5::Digest;
use winapi::{um::{winuser::GetActiveWindow, libloaderapi::{GetModuleHandleA, LoadLibraryA}, d3d11::{D3D11CreateDeviceAndSwapChain, D3D11_SDK_VERSION, ID3D11Device, ID3D11DeviceContext, ID3D11ClassLinkage, ID3D11VertexShader, ID3D11PixelShader}, winnt::DriverType, d3dcommon::D3D_DRIVER_TYPE_HARDWARE}, shared::{minwindef::{HINSTANCE__, HMODULE}, ntdef::{NULL64, HRESULT}, dxgi::{IDXGIAdapter, DXGI_SWAP_CHAIN_DESC, IDXGISwapChain, IDXGIDevice}, dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM, dxgitype::{DXGI_MODE_DESC, DXGI_SAMPLE_DESC, DXGI_RATIONAL, DXGI_USAGE_RENDER_TARGET_OUTPUT}}};

pub type CCreateVertexShaderFn = unsafe fn(*mut ID3D11Device, *const c_void, usize, *mut ID3D11ClassLinkage, *mut *mut ID3D11VertexShader) -> HRESULT;
pub type CCreatePixelShaderFn = unsafe fn(*mut ID3D11Device, *const c_void, usize, *mut ID3D11ClassLinkage, *mut *mut ID3D11PixelShader) -> HRESULT;
static_detour! {
    pub static CreateVertexDetour: unsafe fn(*mut ID3D11Device, *const c_void, usize, *mut ID3D11ClassLinkage, *mut *mut ID3D11VertexShader) -> HRESULT;
    pub static CreatePixelDetour: unsafe fn(*mut ID3D11Device, *const c_void, usize, *mut ID3D11ClassLinkage, *mut *mut ID3D11PixelShader) -> HRESULT;
}

const k_shader_path: &str = "shaders"; 

pub fn detoured_create_vertex_shader(device: *mut ID3D11Device, byte_code: *const c_void, length: usize, linkage: *mut ID3D11ClassLinkage, shader: *mut *mut ID3D11VertexShader) -> HRESULT {
    handle_shader("vert", byte_code, length);
    return unsafe { CreateVertexDetour.call(device, byte_code, length, linkage, shader) };
}

pub fn detoured_create_pixel_shader(device: *mut ID3D11Device, byte_code: *const c_void, length: usize, linkage: *mut ID3D11ClassLinkage, shader: *mut *mut ID3D11PixelShader) -> HRESULT {
    handle_shader("frag", byte_code, length);
    return unsafe { CreatePixelDetour.call(device, byte_code, length, linkage, shader) };
}

fn handle_shader(shader_type: &str, byte_code: *const c_void, length: usize) {
    let byte_code = unsafe{ std::slice::from_raw_parts(byte_code as *const u8, length) };
    let digest = md5::compute(byte_code);
    let dump = true;
    if dump {
        dump_shader(shader_type, digest, byte_code);
    }

    print!("{} {:?}", shader_type, digest);
}

fn dump_shader(shader_type: &str, digest: Digest, byte_code: &[u8]) {
    let filename = format!("{:?}.{}", digest, shader_type);
    let shader_path = Path::new(k_shader_path).join(shader_type);
    if fs::create_dir_all(shader_path.clone()).is_ok() {
        if fs::write(shader_path.join(filename), byte_code).is_ok() {
            //Decompile
        }
    }
}