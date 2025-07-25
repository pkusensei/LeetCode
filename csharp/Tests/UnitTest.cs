using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { "1*2*3", "1+2+3" }, "123", 6)]
    [DataRow(new[] { "2*3+2", "2+3*2" }, "232", 8)]
    public void TestMethod1(string[] exp, string s, int target)
    {
        Assert.IsTrue(exp.Order().SequenceEqual(sol.AddOperators(s, target).Order()));
    }

    [TestMethod]
    public void TestMethod2()
    {
        Assert.AreEqual(0, sol.AddOperators("3456237490", 9191).Count);
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