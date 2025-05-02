using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow("RR.L", "RR.L")]
    [DataRow(".L.R...LR..L..", "LL.RR.LLRRLL..")]
    public void TestMethod1(string d, string exp)
    {
        Assert.AreEqual(exp, sol.PushDominoes(d));
    }

    [TestMethod]
    public void TestMethod2()
    { }

    [TestMethod]
    public void TestMethod3()
    {
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}