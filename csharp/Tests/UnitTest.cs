using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 3, 3, 2, 2, 1 }, new[] { 1, 0, 2, 1, 3 })]
    [DataRow(new[] { 2, 1 }, new[] { 1, 2 })]
    public void TestMethod1(int[] exp, int[] nums)
    {
        Assert.IsTrue(exp.SequenceEqual(sol.SmallestSubarrays(nums)));
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