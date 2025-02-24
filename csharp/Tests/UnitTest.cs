using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 3, 1 }, "[-1,-1]")]
    [DataRow(new[] { 5, 3, 1, 2, 5, 1, 2 }, "[1,3]")]
    [DataRow(new[] { 1, 3, 2, 2, 3, 2, 2, 2, 7 }, "[3,3]")]
    public void TestMethod1(int[] nums, string exp)
    {
        var a = ListNode.Make(nums);
        Assert.AreEqual(exp, sol.NodesBetweenCriticalPoints(a).Print());
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