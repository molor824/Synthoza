using System.Numerics;
using Raylib_cs;

namespace Synthoza;

public class TransformTest : ILoopHandle
{
    private Entity _parent, _child;
    private TransformStorage _transformStorage;

    public const float ParentRotation = 1.0f;
    public const float ChildRotation = 1.0f;

    public TransformTest(EntityStorage storage, TransformStorage transformStorage)
    {
        _parent = storage.CreateNew(new Rectangle(new(100.0f, 20.0f))
        {
            Color = Color.Red
        }, Transform.FromTranslation(new(300.0f, 300.0f)));
        _child = storage.CreateNew(new Rectangle(new(50.0f, 10.0f))
        {
            Color = Color.Blue
        }, Transform.FromTranslation(new(100.0f, 0.0f)), _parent);
        storage.CreateNew(new Rectangle(new(10.0f, 10.0f)) { Color = Color.White },
            Transform.FromTranslation(new(50.0f, 0.0f)), _child);
        _transformStorage = transformStorage;
    }

    public void Update(float deltaTime)
    {
        var parentTransform = _transformStorage.GetTransform(_parent);
        var childTransform = _transformStorage.GetTransform(_child);

        parentTransform.Basis = Complex.FromPolarCoordinates(1.0, ParentRotation * deltaTime) * parentTransform.Basis;
        childTransform.Basis = Complex.FromPolarCoordinates(1.0, ChildRotation * deltaTime) * childTransform.Basis;

        _transformStorage.SetTransform(_parent, parentTransform);
        _transformStorage.SetTransform(_child, childTransform);
    }
}