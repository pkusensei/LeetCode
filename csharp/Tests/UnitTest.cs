using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 5, 4, 2, 1 }, 6)]
    [DataRow(new[] { 4, 2, 2, 3 }, 7)]
    [DataRow(new[] { 1, 100000 }, 100001)]
    public void TestMethod1(int[] nums, int exp)
    {
        var a = ListNode.Make(nums);
        Assert.AreEqual(exp, sol.PairSum(a));
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