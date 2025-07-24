using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 24, 12, 8, 6 }, new[] { 1, 2, 3, 4 })]
    [DataRow(new[] { 0, 0, 9, 0, 0 }, new[] { -1, 1, 0, -3, 3 })]
    public void TestMethod1(int[] exp, int[] nums)
    {
        Assert.IsTrue(exp.SequenceEqual(sol.ProductExceptSelf(nums)));
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