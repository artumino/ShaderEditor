namespace ShaderEditor;
using BepInEx;


[BepInPlugin("org.bepinex.plugins.shadereditor", "ShaderEditor", "0.0.1")]
public class ShaderEditor : BaseUnityPlugin
{
    [System.Runtime.InteropServices.DllImport("shadereditor_native.dll")]
    private static extern System.IntPtr HookShaders();

    private void Awake()
    {
        HookShaders();
    }
}
