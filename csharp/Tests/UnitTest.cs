using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 5, 6 }, new[] { 4, 3, 2, 7, 8, 2, 3, 1 })]
    public void TestMethod1(int[] exp, int[] nums)
    {
        Assert.IsTrue(exp.SequenceEqual(sol.FindDisappearedNumbers(nums)));
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