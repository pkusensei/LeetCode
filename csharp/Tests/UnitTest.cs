using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 1, 2, 3, 10, 4, 2, 3, 5 }, 3)]
    [DataRow(new[] { 5, 4, 3, 2, 1 }, 4)]
    [DataRow(new[] { 1, 2, 3 }, 0)]
    public void TestMethod1(int[] nums, int exp)
    {
        Assert.AreEqual(exp, sol.FindLengthOfShortestSubarray(nums));
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