



using System.Reflection;
using Advent.Utils;


List<Type> dynamicTypes = DynamicDays.GetDynamicTypes();

for (int i = 0; i < 10; i++)
{
    // Get the method info for FetchDataAsync<T>
    MethodInfo method = typeof(InputFetcher)
        .GetMethod("FetchDataAsync")!
        .MakeGenericMethod(dynamicTypes[i]);


    // Invoke the method and get the Task result
    Task task = (Task)method.Invoke(null, new object[] { "https://jsonplaceholder.typicode.com/todos/1" })!;
    await task;

    // Get the Result property from Task<T>
    PropertyInfo resultProperty = task.GetType().GetProperty("Result")!;
    object result = resultProperty.GetValue(task)!;
}


