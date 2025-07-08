using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 1, 2, 3 }, new[] { 1, 3, 2 })]
    [DataRow(new[] { 1, 3, 2 }, new[] { 2, 1, 3 })]
    public void TestMethod1(int[] nums, int[] exp)
    {
        sol.NextPermutation(nums);
        Assert.IsTrue(nums.SequenceEqual(exp));
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