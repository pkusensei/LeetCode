using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(12, true)]
    [DataRow(91, true)]
    [DataRow(21, false)]
    public void TestMethod1(int num, bool exp)
    {
        Assert.AreEqual(exp, sol.CheckPowersOfThree(num));
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