using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 5, 3, 1, 2, 5, 1, 2 }, new[] { 1, 3 })]
    [DataRow(new[] { 2, 2, 1, 3 }, new[] { -1, -1 })]
    public void TestMethod1(int[] nums, int[] exp)
    {
        var a = ListNode.Make(nums);
        Assert.IsTrue(sol.NodesBetweenCriticalPoints(a).SequenceEqual(exp));
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