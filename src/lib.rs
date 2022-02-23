use std::{os::windows::raw::HANDLE, ffi::{CString, CStr, c_void}, ptr, mem, fs::File};
use detour::{static_detour, GenericDetour, Function, Error};
use winapi::{um::{winuser::GetActiveWindow, libloaderapi::{GetModuleHandleA, LoadLibraryA}, d3d11::{D3D11CreateDeviceAndSwapChain, D3D11_SDK_VERSION, ID3D11Device, ID3D11DeviceContext, ID3D11ClassLinkage, ID3D11VertexShader, ID3D11PixelShader}, winnt::DriverType, d3dcommon::D3D_DRIVER_TYPE_HARDWARE}, shared::{minwindef::{HINSTANCE__, HMODULE}, ntdef::{NULL64, HRESULT}, dxgi::{IDXGIAdapter, DXGI_SWAP_CHAIN_DESC, IDXGISwapChain, IDXGIDevice}, dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM, dxgitype::{DXGI_MODE_DESC, DXGI_SAMPLE_DESC, DXGI_RATIONAL, DXGI_USAGE_RENDER_TARGET_OUTPUT}}};

use crate::shaders::{CCreatePixelShaderFn, CCreateVertexShaderFn};
mod shaders;

struct Context {
    d3d11_handle: HANDLE,
    dxgi_handle: HANDLE,
    swapchain: *mut IDXGISwapChain,
    device: *mut ID3D11Device,
    device_context: *mut ID3D11DeviceContext
}

#[no_mangle]
pub extern "C" fn HookShaders() {
    let window_handle = unsafe { GetActiveWindow() };

    let mut context = setup();

    if context.d3d11_handle.is_null() || context.dxgi_handle.is_null() {
        println!("Error while loading DXGI or D3D11, aborting");
        return;
    }

    let swapchain_desc = DXGI_SWAP_CHAIN_DESC {
        BufferCount: 1,
        BufferDesc: DXGI_MODE_DESC{ Width: 0, Height: 0, RefreshRate: DXGI_RATIONAL { Numerator: 1, Denominator: 1 }, Format: DXGI_FORMAT_R8G8B8A8_UNORM, ScanlineOrdering: 0, Scaling: 0 },
        SampleDesc: DXGI_SAMPLE_DESC{ Count: 1, Quality: 0 },
        BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
        OutputWindow: window_handle,
        Windowed: 1,
        SwapEffect: 0,
        Flags: 0,
    };

    let ret = unsafe { D3D11CreateDeviceAndSwapChain(ptr::null_mut(), D3D_DRIVER_TYPE_HARDWARE,
                                                    ptr::null_mut(), 0, ptr::null_mut(), 0,
                                                    D3D11_SDK_VERSION, &swapchain_desc,
                                                    &mut context.swapchain, &mut context.device, ptr::null_mut(), &mut context.device_context) };

    if ret >= 0 {
        apply_hooks(&context).unwrap();
        tear_down(&context);
    }
}

fn setup() -> Context {
    Context {
        d3d11_handle: load_library("d3d11.dll"),
        dxgi_handle: load_library("dxgi.dll"),
        swapchain: std::ptr::null_mut(),
        device: std::ptr::null_mut(),
        device_context: std::ptr::null_mut()
    }
}

fn apply_hooks(contex: &Context) -> Result<(), Error>{
    //Here be dragons
    let vtable = unsafe { (contex.device.as_ref().unwrap()).lpVtbl.as_ref().unwrap() };
    let fn_vertex = unsafe { mem::transmute::<_, CCreateVertexShaderFn>(vtable.CreateVertexShader) };
    println!("Vertex Original {:p}", fn_vertex as *const ());
    println!("Vertex Detoured {:p}", shaders::detoured_create_vertex_shader as *const ());
    
    unsafe { shaders::CreateVertexDetour.initialize(fn_vertex, shaders::detoured_create_vertex_shader)? };
    unsafe { shaders::CreateVertexDetour.enable()? };

    let fn_pixel = unsafe { mem::transmute::<_, CCreatePixelShaderFn>(vtable.CreatePixelShader) };
    println!("Pixel Original {:p}", fn_pixel as *const ());
    println!("Pixel Detoured {:p}", shaders::detoured_create_pixel_shader as *const ());

    unsafe { shaders::CreatePixelDetour.initialize(fn_pixel, shaders::detoured_create_pixel_shader)? };
    unsafe { shaders::CreatePixelDetour.enable()? };

    println!("Successfully hooked functions!");
    Ok(())
}

fn tear_down(contex: &Context) {
    if !contex.device.is_null() {
        unsafe { contex.device.as_ref().unwrap().Release() };
    }
    if !contex.device_context.is_null() {
        unsafe { contex.device_context.as_ref().unwrap().Release() };
    }
    if !contex.swapchain.is_null() {
        unsafe { contex.swapchain.as_ref().unwrap().Release() };
    }  
}

fn load_library(name: &str) -> HANDLE {
    let c_name = CString::new(name).unwrap();
    let mut handle: HANDLE  = unsafe { GetModuleHandleA(c_name.as_ptr()) as HANDLE };
    if handle.is_null() {
        handle = unsafe { LoadLibraryA(c_name.as_ptr()) as HANDLE };
    }
    handle
}