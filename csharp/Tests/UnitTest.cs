using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow("abc", "bcabc")]
    [DataRow("acdb", "cbacdcbc")]
    public void TestMethod1(string exp, string s)
    {
        Assert.AreEqual(exp, sol.RemoveDuplicateLetters(s));
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