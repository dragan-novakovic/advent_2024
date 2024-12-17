using System.Reflection;
using System.Reflection.Emit;


namespace Advent.Utils
{
    public class DynamicDays
    {
        public static List<Type> GetDynamicTypes()
        {
            var types = new List<Type>();

            for (int i = 1; i <= 3; i++)
            {
                // Create a dynamic assembly and module
                var assemblyName = new AssemblyName("DynamicDays");
                var assemblyBuilder = AssemblyBuilder.DefineDynamicAssembly(assemblyName, AssemblyBuilderAccess.Run);
                var moduleBuilder = assemblyBuilder.DefineDynamicModule("DynamicDaysModule");

                // Define a dynamic type with the name "TypeX" where X is the value of i
                string typeName = "Day" + i;
                TypeBuilder typeBuilder = moduleBuilder.DefineType(typeName, TypeAttributes.Public);

                // Create the type
                Type dynamicType = typeBuilder.CreateType();
                types.Add(dynamicType);
                Console.WriteLine($"Created type: {dynamicType.Name}");
            }

            return types;
        }
    }
}
