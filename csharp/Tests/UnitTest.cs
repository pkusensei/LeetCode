using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 5, 4, 3, 2, 1 }, new[] { 1, 2, 3, 4, 5 })]
    [DataRow(new[] { 2, 1 }, new[] { 1, 2 })]
    public void TestMethod1(int[] exp, int[] nums)
    {
        var head = ListNode.Make(nums);
        Assert.IsTrue(exp.SequenceEqual(sol.ReverseList(head)));
        head = ListNode.Make(nums);
        Assert.IsTrue(exp.SequenceEqual(sol.WithRecursion(head)));
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