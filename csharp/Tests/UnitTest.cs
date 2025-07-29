using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 1, 6, 1, 5, 1, 4 }, new[] { 1, 5, 1, 1, 6, 4 })]
    [DataRow(new[] { 2, 3, 1, 3, 1, 2 }, new[] { 1, 3, 2, 2, 3, 1 })]
    public void TestMethod1(int[] exp, int[] nums)
    {
        sol.WiggleSort(nums);
        Assert.IsTrue(exp.SequenceEqual(nums));
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