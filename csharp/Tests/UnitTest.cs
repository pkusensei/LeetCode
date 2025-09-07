using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(8, 2)]
    [DataRow(3, 1)]
    [DataRow(19, 3)]
    [DataRow(183236316, 10101)]
    public void TestMethod1(int exp, int n)
    {
        Assert.AreEqual(exp, sol.CheckRecord(n));
        Assert.AreEqual(exp, sol.WithTwoPhases(n));
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