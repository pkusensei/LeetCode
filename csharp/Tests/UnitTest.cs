using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(4, new[] { 6, 0, 8, 2, 1, 5 })]
    [DataRow(7, new[] { 9, 8, 1, 0, 1, 9, 4, 0, 4, 1 })]
    public void TestMethod1(int a, int[] nums)
    {
        var b = sol.MaxWidthRamp(nums);
        Assert.AreEqual(a, b);
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