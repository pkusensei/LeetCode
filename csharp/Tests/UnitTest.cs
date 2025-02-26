using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 5, 2, 6, 3, 9, 1, 7, 3, 8, 4 }, "[5,6,2,3,9,1,4,8,3,7]")]
    [DataRow(new[] { 1, 1, 0, 6 }, "[1,0,1,6]")]
    [DataRow(new[] { 1, 1, 0, 6, 5 }, "[1,0,1,5,6]")]
    public void TestMethod1(int[] nums, string exp)
    {
        var a = ListNode.Make(nums);
        Assert.AreEqual(exp, sol.ReverseEvenLengthGroups(a).Print());
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