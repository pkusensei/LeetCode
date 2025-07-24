using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 0, 2 }, "2-1-1")]
    [DataRow(new[] { -34, -14, -10, -10, 10 }, "2*3-4*5")]
    public void TestMethod1(int[] exp, string s)
    {
        List<int> v = [.. sol.DiffWaysToCompute(s)];
        v.Sort();
        Assert.IsTrue(exp.SequenceEqual(v));
    }

    [TestMethod]
    public void TestMethod2()
    {
    }

    [TestMethod]
    public void TestMethod3()
    {
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}