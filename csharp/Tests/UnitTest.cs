using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(2, new[] { 1, 3, 4, 2, 2 })]
    [DataRow(3, new[] { 3, 1, 3, 4, 2 })]
    [DataRow(3, new[] { 3, 3, 3, 3, 3 })]
    public void TestMethod1(int exp, int[] nums)
    {
        Assert.AreEqual(exp, sol.FindDuplicate(nums));
        Assert.AreEqual(exp, sol.WithFlip(nums));
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