using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new int[] { 2, 1, 5 }, "[5,5,0]")]
    [DataRow(new int[] { 2, 7, 4, 3, 5 }, "[7,0,5,5,0]")]
    [DataRow(new int[] { 9, 7, 6, 7, 6, 9 }, "[0,9,7,9,9,0]")]
    public void TestMethod1(int[] nums, string c)
    {
        var a = ListNode.Make(nums);
        var b = sol.NextLargerNodes(a);
        Assert.AreEqual(c, b.Print());
    }

    [TestMethod]
    public void TestMethod2()
    {
    }

    [TestMethod]
    public void TestMethod3()
    {
    }
}