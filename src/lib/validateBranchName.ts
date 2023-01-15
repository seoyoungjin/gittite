// replace tauri::command validate_branch_name()

/**
 * validate branch name
 *
 * @param {String} branchName - branch name
 * @return {Boolean} result
 */
export function validateBranchName(branchName: string) {
  const pattern =
    "^(master|main|develop){1}$|^(feature|fix|hotfix|release)/.+$";
  const validBranchNameRegExp = new RegExp(pattern, "g");
  const result = validBranchNameRegExp.test(branchName);
  return result;
}
