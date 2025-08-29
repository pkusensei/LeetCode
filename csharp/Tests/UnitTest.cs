using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 1.0, -1.0, -1.0, 3.0, 5.0, 6.0 }, new[] { 1, 3, -1, -3, 5, 3, 6, 7 }, 3)]
    [DataRow(new[] { 2.0, 3.0, 3.0, 3.0, 2.0, 3.0, 2.0 }, new[] { 1, 2, 3, 4, 2, 3, 1, 4, 2 }, 3)]
    public void TestMethod1(double[] exp, int[] nums, int k)
    {
        Assert.IsTrue(exp.SequenceEqual(sol.MedianSlidingWindow(nums, k)));
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