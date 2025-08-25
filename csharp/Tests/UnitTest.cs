using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(false, 10, 11)]
    [DataRow(true, 10, 0)]
    [DataRow(true, 10, 1)]
    [DataRow(false, 10, 40)]
    public void TestMethod1(bool exp, int max, int total)
    {
        Assert.AreEqual(exp, sol.CanIWin(max, total));
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