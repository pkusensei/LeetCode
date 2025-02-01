using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 1, 2, 3, 4, 5 }, 2, "[1,4,3,2,5]")]
    [DataRow(new[] { 7, 9, 6, 6, 7, 8, 3, 0, 9, 5 }, 5, "[7,9,6,6,8,7,3,0,9,5]")]
    public void TestMethod1(int[] nums, int k, string exp)
    {
        var a = ListNode.Make(nums);
        Assert.AreEqual(exp, sol.SwapNodes(a, k).ToString());
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