using Raylib_cs;

namespace Synthoza;

internal static class Program
{
    private static void Main(string[] args)
    {
        var app = new App("Synthoza", 1280, 720);

        var renderPass = new RenderPass();
        var pianoRoll = new PianoRoll();
        
        renderPass.AddHandles(pianoRoll);
        app.AddHandles(pianoRoll, renderPass);
        
        app.Run();
        
        Raylib.CloseWindow();
    }
}