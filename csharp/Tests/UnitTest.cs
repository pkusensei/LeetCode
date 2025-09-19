using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow("x=2", "x+5-3+x=6+x-2")]
    [DataRow("Infinite solutions", "x=x")]
    [DataRow("x=0", "2x=x")]
    [DataRow("x=1", "-x=-1")]
    [DataRow("x=-2", "2=-x")]
    [DataRow("Infinite solutions", "0x=0")]
    public void TestMethod1(string exp, string s)
    {
        Assert.AreEqual(exp, sol.SolveEquation(s));
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