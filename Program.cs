using Raylib_cs;

namespace Synthoza;

internal static class Program
{
    private static void Main(string[] args)
    {
        var app = new App("Synthoza", 1280, 720);

        var renderPass = new RenderPass();
        var cameraPass = new CameraPass(app);
        var pianoRoll = new PianoRoll(cameraPass);
        
        cameraPass.AddHandle(pianoRoll);
        renderPass.AddHandles(cameraPass);
        app.AddHandles(renderPass);
        
        app.Run();
        
        Raylib.CloseWindow();
    }
}