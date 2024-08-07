using System.Diagnostics;
using System.Text;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = NumTrees(3);
    var a = 5;
    Debug.Assert(n == a, $"Output: {n}\nExpect: {a}");
}

void Test2()
{
}

int NumTrees(int n)
{
    var values = new int[n + 1];
    values[0] = 1;
    values[1] = 1;
    return Solve(n, values);
}

int Solve(int n, int[] values)
{
    if (values[n] != 0)
    {
        return values[n];
    }

    var res = 0;
    for (int i = 0; i < n; i++)
    {
        res += Solve(i, values) * Solve(n - i - 1, values);
    }
    values[n] = res;
    return res;
}

string Print<T>(IList<T> values)
{
    var sb = new StringBuilder();
    sb.Append('[');
    sb.AppendJoin(',', values);
    sb.Append([']']);
    return sb.ToString();
}
