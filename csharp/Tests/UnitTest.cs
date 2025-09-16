using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 3, 4 }, 2, new[] { "0:start:0", "1:start:2", "1:end:5", "0:end:6" })]
    [DataRow(new[] { 8 }, 1, new[] { "0:start:0", "0:start:2", "0:end:5", "0:start:6", "0:end:6", "0:end:7" })]
    [DataRow(new[] { 7, 1 }, 2, new[] { "0:start:0", "0:start:2", "0:end:5", "1:start:6", "1:end:6", "0:end:7" })]
    public void TestMethod1(int[] exp, int n, string[] s)
    {
        Assert.IsTrue(exp.SequenceEqual(sol.ExclusiveTime(n, s)));
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