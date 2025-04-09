using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 1, 8, 9 }, "[3,7,8]")]
    [DataRow(new[] { 9, 9, 9 }, "[1,9,9,8]")]
    public void TestMethod1(int[] nums, string exp)
    {
        var a = ListNode.Make(nums);
        Assert.AreEqual(exp, sol.DoubleIt(a).ToString());
    }

    [TestMethod]
    public void TestMethod2()
    { }

    [TestMethod]
    public void TestMethod3()
    {
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}