using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(0, new[] { 0 })]
    [DataRow(3, new[] { 0, 1, 3, 5, 6 })]
    public void TestMethod1(int exp, int[] c)
    {
        Assert.AreEqual(exp, sol.HIndex_2(c));
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