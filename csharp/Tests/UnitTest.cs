using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(6, new[] { 2, 3, -2, 4 })]
    [DataRow(0, new[] { -2, 0, -1 })]
    [DataRow(960, new[] { -1, 4, -4, 5, -2, -1, -1, -2, -3 })]
    public void TestMethod1(int exp, int[] nums)
    {
        Assert.AreEqual(exp, sol.Kadanes(nums));
        Assert.AreEqual(exp, sol.WithPrefSuf(nums));
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