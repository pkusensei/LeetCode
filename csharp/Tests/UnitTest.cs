using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 3, 2, 4 }, 2, 1, 3)]
    [DataRow(new[] { 3, 1, 9, 6 }, 3, 0, 2)]
    public void TestMethod1(int[] nums, int modulo, int k, int exp)
    {
        Assert.AreEqual(exp, sol.CountInterestingSubarrays(nums, modulo, k));
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