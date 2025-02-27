using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 1, 3, 4, 7, 1, 2, 6 }, "[1,3,4,1,2,6]")]
    [DataRow(new[] { 1, 2, 3, 4 }, "[1,2,4]")]
    [DataRow(new[] { 1, 2 }, "[1]")]
    public void TestMethod1(int[] nums, string exp)
    {
        var a = ListNode.Make(nums);
        Assert.AreEqual(exp, sol.DeleteMiddle(a).ToString());
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