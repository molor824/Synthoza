using System.Reflection;
using System.Reflection.Emit;

namespace Synthoza.Sylan;

public class Compiler
{
    private AssemblyBuilder _asmBuilder;
    private ModuleBuilder _instModuleBuilder;

    public Compiler()
    {
        _asmBuilder =
            AssemblyBuilder.DefineDynamicAssembly(new("<SYLAN_COMPILED_ASSEMBLY>"),
                AssemblyBuilderAccess.RunAndCollect);
        _instModuleBuilder = _asmBuilder.DefineDynamicModule("Instrument");
    }

    static MethodInfo EmitDefaultGetter(TypeBuilder typeBuilder, PropertyBuilder property, FieldInfo backingField)
    {
        var getterMethod = typeBuilder.DefineMethod("get_" + property.Name,
            MethodAttributes.HideBySig | MethodAttributes.Public | MethodAttributes.SpecialName,
            backingField.FieldType,
            []
        );
        var ilgen = getterMethod.GetILGenerator();
        ilgen.Emit(OpCodes.Ldarg_0);
        ilgen.Emit(OpCodes.Ldfld, backingField);
        ilgen.Emit(OpCodes.Ret);
        
        property.SetGetMethod(getterMethod);
        return getterMethod;
    }

    static MethodInfo EmitDefaultSetter(TypeBuilder typeBuilder, PropertyBuilder property, FieldInfo backingField)
    {
        var setterMethod = typeBuilder.DefineMethod("set_" + property.Name,
            MethodAttributes.HideBySig | MethodAttributes.Public | MethodAttributes.SpecialName,
            null,
            [backingField.FieldType]
        );
        var ilgen = setterMethod.GetILGenerator();
        ilgen.Emit(OpCodes.Ldarg_0);
        ilgen.Emit(OpCodes.Ldarg_1);
        ilgen.Emit(OpCodes.Stfld, backingField);
        ilgen.Emit(OpCodes.Ret);
        
        property.SetSetMethod(setterMethod);
        return setterMethod;
    }

    static (PropertyBuilder, FieldBuilder) EmitBackedProperty(TypeBuilder typeBuilder, string name, Type propertyType)
    {
        var property = typeBuilder.DefineProperty(name, PropertyAttributes.None, propertyType, null);
        var backingField = typeBuilder.DefineField("<backing>__" + name, propertyType, FieldAttributes.Private);
        return (property, backingField);
    }

    public Type DefineInstrumentClass(string className)
    {
        var typeBuilder = _instModuleBuilder.DefineType(className, TypeAttributes.AutoClass | TypeAttributes.Public,
            typeof(Instrument));

        var baseFrequencyMethod =
            typeof(Instrument).GetMethod("Frequency", BindingFlags.NonPublic | BindingFlags.Instance)!;
        var baseWaveMethod = typeof(Instrument).GetMethod("Wave", BindingFlags.NonPublic | BindingFlags.Instance)!;

        var (periodProperty, periodField) = EmitBackedProperty(typeBuilder, "Period", typeof(double));

        var periodGetter = EmitDefaultGetter(typeBuilder, periodProperty, periodField);
        var periodSetter = EmitDefaultSetter(typeBuilder, periodProperty, periodField);
        
        var (dutyCycleProperty, dutyCycleField) = EmitBackedProperty(typeBuilder, "DutyCycle", typeof(double));

        var dutyCycleGetter = EmitDefaultGetter(typeBuilder, dutyCycleProperty, dutyCycleField);
        var dutyCycleSetter = EmitDefaultSetter(typeBuilder, dutyCycleProperty, dutyCycleField);

        var frequencyMethodBuilder = typeBuilder.DefineMethod(baseFrequencyMethod.Name,
            MethodAttributes.Family | MethodAttributes.Virtual | MethodAttributes.HideBySig,
            baseFrequencyMethod.ReturnType, baseFrequencyMethod.GetParameters().Select(p => p.ParameterType).ToArray()
        );
        var ilGen = frequencyMethodBuilder.GetILGenerator();
        var condMet1 = ilGen.DefineLabel();
        var condBreak1 = ilGen.DefineLabel();
        ilGen.Emit(OpCodes.Ldarg_1);
        ilGen.Emit(OpCodes.Call, ((Func<double, double>)Builtin.PitchToFreq).Method);
        ilGen.Emit(OpCodes.Ldarg_2);
        ilGen.Emit(OpCodes.Ldarg_0);
        ilGen.Emit(OpCodes.Call, periodGetter);
        ilGen.Emit(OpCodes.Rem);
        ilGen.Emit(OpCodes.Ldarg_0);
        ilGen.Emit(OpCodes.Call, periodGetter);
        ilGen.Emit(OpCodes.Ldarg_0);
        ilGen.Emit(OpCodes.Call, dutyCycleGetter);
        ilGen.Emit(OpCodes.Mul);
        ilGen.Emit(OpCodes.Blt, condMet1);
        ilGen.Emit(OpCodes.Ldc_R8, 2.0);
        ilGen.Emit(OpCodes.Br, condBreak1);
        ilGen.MarkLabel(condMet1);
        ilGen.Emit(OpCodes.Ldc_R8, 1.0);
        ilGen.MarkLabel(condBreak1);
        ilGen.Emit(OpCodes.Mul);
        ilGen.Emit(OpCodes.Ret);

        typeBuilder.DefineMethodOverride(frequencyMethodBuilder, baseFrequencyMethod);

        var waveMethodBuilder = typeBuilder.DefineMethod(baseWaveMethod.Name,
            MethodAttributes.Family | MethodAttributes.Virtual | MethodAttributes.HideBySig,
            baseWaveMethod.ReturnType, baseWaveMethod.GetParameters().Select(p => p.ParameterType).ToArray()
        );
        ilGen = waveMethodBuilder.GetILGenerator();
        var condMet2 = ilGen.DefineLabel();
        var condBreak2 = ilGen.DefineLabel();
        ilGen.Emit(OpCodes.Ldarg_1);
        ilGen.Emit(OpCodes.Ldc_R8, 0.5);
        ilGen.Emit(OpCodes.Blt, condMet2);
        ilGen.Emit(OpCodes.Ldarg_1);
        ilGen.Emit(OpCodes.Ldc_R8, -4.0);
        ilGen.Emit(OpCodes.Mul);
        ilGen.Emit(OpCodes.Ldc_R8, 3.0);
        ilGen.Emit(OpCodes.Add);
        ilGen.Emit(OpCodes.Br, condBreak2);
        ilGen.MarkLabel(condMet2);
        ilGen.Emit(OpCodes.Ldarg_1);
        ilGen.Emit(OpCodes.Ldc_R8, 4.0);
        ilGen.Emit(OpCodes.Mul);
        ilGen.Emit(OpCodes.Ldc_R8, 1.0);
        ilGen.Emit(OpCodes.Sub);
        ilGen.MarkLabel(condBreak2);
        ilGen.Emit(OpCodes.Ret);

        typeBuilder.DefineMethodOverride(waveMethodBuilder, baseWaveMethod);

        var ctor1Builder = typeBuilder.DefineConstructor(MethodAttributes.Public, CallingConventions.HasThis,
            [typeof(double), typeof(double)]);
        ilGen = ctor1Builder.GetILGenerator();
        ilGen.Emit(OpCodes.Ldarg_0);
        ilGen.Emit(OpCodes.Ldarg_1);
        ilGen.Emit(OpCodes.Call, periodSetter);
        ilGen.Emit(OpCodes.Ldarg_0);
        ilGen.Emit(OpCodes.Ldarg_2);
        ilGen.Emit(OpCodes.Call, dutyCycleSetter);
        ilGen.Emit(OpCodes.Ldarg_0);
        ilGen.Emit(OpCodes.Call,
            typeof(Instrument).GetConstructor(BindingFlags.NonPublic | BindingFlags.Instance, [])!);
        ilGen.Emit(OpCodes.Ret);

        var ctor2Builder = typeBuilder.DefineConstructor(MethodAttributes.Public, CallingConventions.HasThis, []);
        ilGen = ctor2Builder.GetILGenerator();
        ilGen.Emit(OpCodes.Ldarg_0);
        ilGen.Emit(OpCodes.Ldc_R8, 0.05);
        ilGen.Emit(OpCodes.Ldc_R8, 0.4);
        ilGen.Emit(OpCodes.Call, ctor1Builder);
        ilGen.Emit(OpCodes.Ret);

        return typeBuilder.CreateType();
    }
}

public static class Builtin
{
    public static readonly double A0Frequency = 27.5;

    public static double PitchToFreq(double pitch)
    {
        var baseFrequency1 = A0Frequency;
        return baseFrequency1 * Math.Pow(2, pitch);
    }
}