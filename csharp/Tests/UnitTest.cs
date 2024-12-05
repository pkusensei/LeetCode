using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new int[] { 1, 2, -3, 3, 1 }, "[3,1]")]
    [DataRow(new int[] { 1, 2, 3, -3, 4 }, "[1,2,4]")]
    [DataRow(new int[] { 1, 2, 3, -3, -2 }, "[1]")]
    [DataRow(new int[] { 1, 3, 2, -3, -2, 5, 5, -5, 1 }, "[1,5,1]")]
    public void TestMethod1(int[] nums, string exp)
    {
        var a = ListNode.Make(nums);
        Assert.AreEqual(exp, sol.RemoveZeroSumSublists(a).ToString());
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