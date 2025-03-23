using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 5, 2, 13, 3, 8 }, "[13,8]")]
    [DataRow(new[] { 1, 1, 1, 1 }, "[1,1,1,1]")]
    public void TestMethod1(int[] nums, string exp)
    {
        var a = ListNode.Make(nums);
        Assert.AreEqual(exp, sol.RemoveNodes(a).ToString());
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